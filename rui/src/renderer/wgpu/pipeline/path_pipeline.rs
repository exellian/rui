use crate::renderer::wgpu::primitive;
use crate::renderer::wgpu::primitive::Paths;
use crate::util::PathSegment;
use alloc::borrow::Cow;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu_types::BufferUsages;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub rect: [f32; 4],
    pub color: [f32; 4],
    pub segment_range: [u32; 2],
}
impl Instance {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Uint32x2,
                },
            ],
        }
    }
}

pub struct Path {
    path: primitive::Path,
    buffer: wgpu::Buffer,
}

pub struct PathPipeline {
    pipeline: wgpu::RenderPipeline,
    globals_buffer: wgpu::Buffer,
    globals_bind_group: wgpu::BindGroup,
    instance_count: usize,
    instance_buffer: Option<wgpu::Buffer>,
    segments_buffer: Option<wgpu::Buffer>,
    segments_bind_group: Option<wgpu::BindGroup>,
    segments_bind_group_layout: wgpu::BindGroupLayout,
}
impl PathPipeline {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let globals = primitive::Globals {
            aspect_ratio: config.width as f32 / config.height as f32,
        };

        let globals_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("globals_buffer"),
            contents: bytemuck::cast_slice(&[globals]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let globals_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("globals_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let globals_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &globals_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: globals_buffer.as_entire_binding(),
            }],
            label: Some("globals_bind_group"),
        });

        let segments_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("segments_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../../../../shader/path.wgsl"
            ))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&globals_bind_group_layout, &segments_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Instance::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[config.format.into()],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        PathPipeline {
            pipeline,
            globals_buffer,
            globals_bind_group,
            instance_count: 0,
            instance_buffer: None,
            segments_buffer: None,
            segments_bind_group: None,
            segments_bind_group_layout,
        }
    }

    pub fn resize(&self, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration) {
        let globals = primitive::Globals {
            aspect_ratio: config.width as f32 / config.height as f32,
        };
        queue.write_buffer(&self.globals_buffer, 0, bytemuck::cast_slice(&[globals]));
    }

    pub fn record<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.instance_count > 0 {
            // If instance_count > 0 then the instance buffer must exist
            debug_assert!(self.instance_buffer.is_some());
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.globals_bind_group, &[]);
            render_pass.set_bind_group(1, self.segments_bind_group.as_ref().unwrap(), &[]);
            render_pass.set_vertex_buffer(0, self.instance_buffer.as_ref().unwrap().slice(..));
            render_pass.draw(0..6, 0..self.instance_count as _);
        }
    }

    pub(crate) fn mount(&mut self, device: &wgpu::Device, paths: &Vec<primitive::Path>) {
        if paths.len() == 0 {
            return;
        }
        let mut all_paths: primitive::Paths = bytemuck::Zeroable::zeroed();
        let mut instances = Vec::with_capacity(paths.len());
        let mut index = 0;
        for p in paths {
            let start = index;
            for s in &p.segments {
                all_paths.segments[index as usize] = *s;
                index += 1;
            }
            instances.push(Instance {
                rect: p.rect,
                color: p.color,
                segment_range: [start, index],
            })
        }
        // Use write buffer instead of recreation in the future.
        //      queue.write_buffer(&self.instance_buffer, offset, updated_data)
        // But this requires memory management.
        // For minimal working version recreate the buffer every frame
        self.instance_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: BufferUsages::VERTEX,
        }));
        self.instance_count = instances.len();

        self.segments_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Segments Buffer"),
            contents: bytemuck::cast_slice(&[all_paths]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        }));

        self.segments_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.segments_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.segments_buffer.as_ref().unwrap().as_entire_binding(),
            }],
            label: Some("segments_bind_group"),
        }));
    }
}
