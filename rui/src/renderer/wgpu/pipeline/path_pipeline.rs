use crate::math::{max, min, rect};
use crate::renderer::wgpu::primitive;
use crate::renderer::wgpu::primitive::PathSegment;
use crate::renderer::MSAA;
use crate::util::Rect;
use crate::{math, util};
use alloc::borrow::Cow;
use rui_util::{be, bs};
use std::mem;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Device, Texture};
use wgpu_types::{
    BufferUsages, Extent3d, MultisampleState, StorageTextureAccess, TextureFormat,
    TextureSampleType, TextureUsages, TextureViewDimension,
};

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

pub struct PathPipeline {
    compute_pipeline: wgpu::ComputePipeline,
    pipeline: wgpu::RenderPipeline,
    globals_buffer: wgpu::Buffer,
    globals_bind_group: wgpu::BindGroup,
    path_segment_count: u32,
    instance_count: usize,
    instance_buffer: Option<wgpu::Buffer>,
    segments_buffer: Option<wgpu::Buffer>,
    segments_bind_group: Option<wgpu::BindGroup>,
    segments_bind_group_layout: wgpu::BindGroupLayout,
    input_texture_bind_group: wgpu::BindGroup,
    input_texture_bind_group_layout: wgpu::BindGroupLayout,
    output_texture_bind_group: wgpu::BindGroup,
    output_texture_bind_group_layout: wgpu::BindGroupLayout,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    texture_size: Extent3d,
}
impl PathPipeline {
    const MAX_PATH_SEGMENTS: u32 = 4096;

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
                    visibility: wgpu::ShaderStages::VERTEX
                        | wgpu::ShaderStages::FRAGMENT
                        | wgpu::ShaderStages::COMPUTE,
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
                    visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let texture_size = Extent3d {
            width: config.width * 2,
            // For now use limited amount of path segments
            height: Self::MAX_PATH_SEGMENTS,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            size: texture_size,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba32Float,
            // Used as output from the compute shader therefore STORAGE_BINDING
            // And used as input texture for the next pipeline
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("texture"),
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let input_texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                }],
                label: Some("input_texture_bind_group_layout"),
            });

        let output_texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: StorageTextureAccess::WriteOnly,
                        format: TextureFormat::Rgba32Float,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                }],
                label: Some("output_texture_bind_group_layout"),
            });

        // One bind group for the input for the path pipeline
        let input_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &input_texture_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            }],
            label: Some("input_texture_bind_group"),
        });

        // And another bind group for the output of the path_compute pipeline
        let output_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &output_texture_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            }],
            label: Some("output_texture_bind_group"),
        });

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../../../../shader/path.wgsl"
            ))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &globals_bind_group_layout,
                &segments_bind_group_layout,
                &input_texture_bind_group_layout,
            ],
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
            multisample,
            multiview: None,
        });

        let compute_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../../../../shader/path_compute.wgsl"
            ))),
        });

        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &globals_bind_group_layout,
                    &segments_bind_group_layout,
                    &output_texture_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "cs_main",
        });

        PathPipeline {
            compute_pipeline,
            pipeline,
            globals_buffer,
            globals_bind_group,
            path_segment_count: 0,
            instance_count: 0,
            instance_buffer: None,
            segments_buffer: None,
            segments_bind_group: None,
            segments_bind_group_layout,
            input_texture_bind_group,
            input_texture_bind_group_layout,
            output_texture_bind_group,
            output_texture_bind_group_layout,
            texture,
            texture_view,
            texture_size,
        }
    }

    pub fn resize(
        &mut self,
        device: &Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) {
        let globals = primitive::Globals {
            width_height: util::pack(config.width as u16, config.height as u16),
            aspect_ratio: config.width as f32 / config.height as f32,
        };

        queue.write_buffer(&self.globals_buffer, 0, bytemuck::cast_slice(&[globals]));

        // Allocating always the double size of the compute texture and only resizing if the double size exceeded
        if config.width <= self.texture_size.width {
            return;
        }
        // Recreate the texture and all bind groups which hold a reference to the texture view
        let texture_size = wgpu::Extent3d {
            width: config.width * 2,
            // For now use limited amount of path segments
            height: Self::MAX_PATH_SEGMENTS,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            size: texture_size,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba32Float,
            // Used as output from the compute shader therefore STORAGE_BINDING
            // And used as input texture for the next pipeline
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
            label: Some("compute_texture"),
        });
        let texture_view = self
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // One bind group for the input for the path pipeline
        self.input_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.input_texture_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            }],
            label: Some("input_texture_bind_group"),
        });

        // And another bind group for the output of the path_compute pipeline
        self.output_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.output_texture_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            }],
            label: Some("output_texture_bind_group"),
        });

        self.texture_view = texture_view;
        self.texture = texture;
        self.texture_size = texture_size;
    }

    pub fn record_compute<'a>(&'a self, compute_pass: &mut wgpu::ComputePass<'a>) {
        if self.path_segment_count > 0 {
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.globals_bind_group, &[]);
            compute_pass.set_bind_group(1, self.segments_bind_group.as_ref().unwrap(), &[]);
            compute_pass.set_bind_group(2, &self.output_texture_bind_group, &[]);
            compute_pass.dispatch(self.path_segment_count as u32, 1, 1);
        }
    }

    pub fn record<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.instance_count > 0 {
            // If instance_count > 0 then the instance buffer must exist
            debug_assert!(self.instance_buffer.is_some());
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.globals_bind_group, &[]);
            render_pass.set_bind_group(1, self.segments_bind_group.as_ref().unwrap(), &[]);
            render_pass.set_bind_group(2, &self.input_texture_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.instance_buffer.as_ref().unwrap().slice(..));
            render_pass.draw(0..6, 0..self.instance_count as _);
        }
    }

    pub(crate) fn mount(&mut self, device: &wgpu::Device, paths: Vec<primitive::Path>) {
        if paths.len() == 0 {
            return;
        }
        let mut all_paths = Vec::new();
        let mut instances = Vec::with_capacity(paths.len());
        let mut index = 0;
        for mut p in paths {
            let start = index;
            let mut rect = [1.0, 1.0, 0.0, 0.0].into();
            for s in &mut p.segments {
                let segment_rect = match s.typ {
                    PathSegment::CUBIC_BEZIER => math::solve_minmax_cubic_bezier(
                        s.param0.into(),
                        s.param1.into(),
                        s.param2.into(),
                        s.param3.into(),
                    ),
                    PathSegment::LINEAR => rect::rect(s.param0.into(), s.param1.into()),
                    _ => unreachable!(),
                };
                s.rect_lu = [segment_rect[0], segment_rect[1]];
                s.rect_rl = [segment_rect[2], segment_rect[3]];

                // Set offset to current sum of widths of previous path segments
                all_paths.push(*s);

                // Increment index and offset and path bounding rect
                rect = rect::max(rect, segment_rect);
                index += 1;
            }
            rect = math::rect::min(rect, p.rect.into());
            instances.push(Instance {
                rect: rect.into(),
                color: p.color,
                segment_range: [start, index],
            });
        }
        if index > Self::MAX_PATH_SEGMENTS {
            panic!("Reached maximum amount of path segments!");
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
        self.path_segment_count = index;
        self.instance_count = instances.len();

        self.segments_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Segments Buffer"),
            contents: bytemuck::cast_slice(all_paths.as_slice()),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
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
