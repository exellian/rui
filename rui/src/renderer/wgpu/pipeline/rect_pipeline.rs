use crate::renderer::wgpu::primitive;
use crate::renderer::MSAA;
use crate::util;
use std::borrow::Cow;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu_types::{BufferUsages, MultisampleState};

pub struct RectPipeline {
    pipeline: wgpu::RenderPipeline,
    globals_buffer: wgpu::Buffer,
    globals_bind_group: wgpu::BindGroup,
    instance_count: usize,
    instance_buffer: Option<wgpu::Buffer>,
}
impl RectPipeline {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, msaa: &MSAA) -> Self {
        let globals = primitive::Globals {
            width_height: util::pack(config.width as u16, config.height as u16),
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

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../../../../shader/rect.wgsl"
            ))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&globals_bind_group_layout],
            push_constant_ranges: &[],
        });

        let multisample = wgpu::MultisampleState {
            count: msaa.clone().into(),
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[primitive::Instance::desc()],
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
            multisample,
            multiview: None,
        });

        RectPipeline {
            pipeline,
            globals_buffer,
            globals_bind_group,
            instance_count: 0,
            instance_buffer: None,
        }
    }

    pub fn resize(&self, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration) {
        let globals = primitive::Globals {
            width_height: util::pack(config.width as u16, config.height as u16),
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
            render_pass.set_vertex_buffer(0, self.instance_buffer.as_ref().unwrap().slice(..));
            render_pass.draw(0..6, 0..self.instance_count as _);
        }
    }

    pub(crate) fn mount(&mut self, device: &wgpu::Device, rects: &Vec<primitive::Rect>) {
        if rects.len() == 0 {
            return;
        }
        // Use write buffer instead of recreation in the future.
        //      queue.write_buffer(&self.instance_buffer, offset, updated_data)
        // But this requires memory management.
        // For minimal working version recreate the buffer every frame
        self.instance_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(rects),
            usage: BufferUsages::VERTEX,
        }));
        self.instance_count = rects.len();
    }
}
