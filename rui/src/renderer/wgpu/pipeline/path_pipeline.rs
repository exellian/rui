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
use wgpu::LoadOp;
use wgpu_types::{
    BufferUsages, CompareFunction, DepthStencilState, Extent3d, MultisampleState, StencilFaceState,
    StencilOperation, StencilState, StorageTextureAccess, TextureFormat, TextureSampleType,
    TextureUsages, TextureViewDimension,
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

struct PathBundle {
    capacity: u32,

    path_segment_count: u32,
    instance_count: usize,
    instance_buffer: Option<wgpu::Buffer>,

    segments_buffer: Option<wgpu::Buffer>,
    segments_bind_group: Option<wgpu::BindGroup>,

    solutions_texture_x: wgpu::Texture,
    solutions_texture_x_view: wgpu::TextureView,
    area_directions_texture_x: wgpu::Texture,
    area_directions_texture_x_view: wgpu::TextureView,

    solutions_texture_y: wgpu::Texture,
    solutions_texture_y_view: wgpu::TextureView,
    area_directions_texture_y: wgpu::Texture,
    area_directions_texture_y_view: wgpu::TextureView,

    input_texture_bind_group_x: wgpu::BindGroup,
    input_texture_bind_group_y: wgpu::BindGroup,
    output_texture_bind_group_x: wgpu::BindGroup,
    output_texture_bind_group_y: wgpu::BindGroup,

    texture_size_x: Extent3d,
    texture_size_y: Extent3d,
}

impl PathBundle {
    fn new(
        capacity: u32,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        input_texture_bind_group_layout: &wgpu::BindGroupLayout,
        output_texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let texture_size_x = Extent3d {
            width: config.height * 2,
            // For now use limited amount of path segments
            height: capacity,
            depth_or_array_layers: 1,
        };
        let texture_size_y = Extent3d {
            width: config.width * 2,
            // For now use limited amount of path segments
            height: capacity,
            depth_or_array_layers: 1,
        };
        let solutions_texture_x = device.create_texture(&wgpu::TextureDescriptor {
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            size: texture_size_x,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba32Float,
            // Used as output from the compute shader therefore STORAGE_BINDING
            // And used as input texture for the next pipeline
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("solutions_texture_x"),
        });
        let solutions_texture_x_view =
            solutions_texture_x.create_view(&wgpu::TextureViewDescriptor::default());

        let area_directions_texture_x = device.create_texture(&wgpu::TextureDescriptor {
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            size: texture_size_x,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba32Uint,
            // Used as output from the compute shader therefore STORAGE_BINDING
            // And used as input texture for the next pipeline
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("gradient_texture_x"),
        });
        let area_directions_texture_x_view =
            area_directions_texture_x.create_view(&wgpu::TextureViewDescriptor::default());

        let solutions_texture_y = device.create_texture(&wgpu::TextureDescriptor {
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            size: texture_size_y,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba32Float,
            // Used as output from the compute shader therefore STORAGE_BINDING
            // And used as input texture for the next pipeline
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("solutions_texture_y"),
        });
        let solutions_texture_y_view =
            solutions_texture_y.create_view(&wgpu::TextureViewDescriptor::default());

        let area_directions_texture_y = device.create_texture(&wgpu::TextureDescriptor {
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            size: texture_size_y,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba32Uint,
            // Used as output from the compute shader therefore STORAGE_BINDING
            // And used as input texture for the next pipeline
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("gradient_texture_y"),
        });
        let area_directions_texture_y_view =
            area_directions_texture_y.create_view(&wgpu::TextureViewDescriptor::default());

        // One bind group for the input for the path pipeline
        let input_texture_bind_group_x = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: input_texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&solutions_texture_x_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&area_directions_texture_x_view),
                },
            ],
            label: Some("input_texture_bind_group_x"),
        });
        let input_texture_bind_group_y = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: input_texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&solutions_texture_y_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&area_directions_texture_y_view),
                },
            ],
            label: Some("input_texture_bind_group_y"),
        });

        // And another bind group for the output of the path_compute pipeline
        let output_texture_bind_group_x = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: output_texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&solutions_texture_x_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&area_directions_texture_x_view),
                },
            ],
            label: Some("output_texture_bind_group_x"),
        });
        let output_texture_bind_group_y = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: output_texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&solutions_texture_y_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&area_directions_texture_y_view),
                },
            ],
            label: Some("output_texture_bind_group_y"),
        });
        PathBundle {
            capacity,
            path_segment_count: 0,
            instance_count: 0,
            instance_buffer: None,
            segments_buffer: None,
            segments_bind_group: None,
            solutions_texture_x,
            solutions_texture_x_view,
            solutions_texture_y,
            solutions_texture_y_view,
            area_directions_texture_x,
            area_directions_texture_x_view,
            area_directions_texture_y,
            area_directions_texture_y_view,
            input_texture_bind_group_x,
            input_texture_bind_group_y,
            output_texture_bind_group_x,
            output_texture_bind_group_y,
            texture_size_x,
            texture_size_y,
        }
    }

    fn mount(
        &mut self,
        device: &wgpu::Device,
        paths: Vec<primitive::Path>,
        segments_bind_group_layout: &wgpu::BindGroupLayout,
    ) {
        debug_assert!(paths.len() <= self.capacity as usize);
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
        if index > self.capacity {
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
            layout: segments_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.segments_buffer.as_ref().unwrap().as_entire_binding(),
            }],
            label: Some("segments_bind_group"),
        }));
    }

    fn resize(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
        input_texture_bind_group_layout: &wgpu::BindGroupLayout,
        output_texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) {
        if config.width > self.texture_size_y.width {
            let texture_size_y = Extent3d {
                width: config.width * 2,
                // For now use limited amount of path segments
                height: self.capacity,
                depth_or_array_layers: 1,
            };

            let solutions_texture_y = device.create_texture(&wgpu::TextureDescriptor {
                // All textures are stored as 3D, we represent our 2D texture
                // by setting depth to 1.
                size: texture_size_y,
                mip_level_count: 1, // We'll talk about this a little later
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // Most images are stored using sRGB so we need to reflect that here.
                format: wgpu::TextureFormat::Rgba32Float,
                // Used as output from the compute shader therefore STORAGE_BINDING
                // And used as input texture for the next pipeline
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                label: Some("solutions_texture_y"),
            });
            let solutions_texture_y_view =
                solutions_texture_y.create_view(&wgpu::TextureViewDescriptor::default());

            let area_directions_texture_y = device.create_texture(&wgpu::TextureDescriptor {
                // All textures are stored as 3D, we represent our 2D texture
                // by setting depth to 1.
                size: texture_size_y,
                mip_level_count: 1, // We'll talk about this a little later
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // Most images are stored using sRGB so we need to reflect that here.
                format: wgpu::TextureFormat::Rgba32Uint,
                // Used as output from the compute shader therefore STORAGE_BINDING
                // And used as input texture for the next pipeline
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                label: Some("gradient_texture_y"),
            });
            let area_directions_texture_y_view =
                area_directions_texture_y.create_view(&wgpu::TextureViewDescriptor::default());

            let input_texture_bind_group_y = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: input_texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&solutions_texture_y_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(
                            &area_directions_texture_y_view,
                        ),
                    },
                ],
                label: Some("input_texture_bind_group_y"),
            });

            let output_texture_bind_group_y =
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: output_texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&solutions_texture_y_view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(
                                &area_directions_texture_y_view,
                            ),
                        },
                    ],
                    label: Some("output_texture_bind_group_y"),
                });

            self.texture_size_y = texture_size_y;
            self.solutions_texture_y = solutions_texture_y;
            self.solutions_texture_y_view = solutions_texture_y_view;
            self.area_directions_texture_y = area_directions_texture_y;
            self.area_directions_texture_y_view = area_directions_texture_y_view;
            self.input_texture_bind_group_y = input_texture_bind_group_y;
            self.output_texture_bind_group_y = output_texture_bind_group_y;
        }
        // The width property of texture_size_x represents the height
        if config.height > self.texture_size_x.width {
            let texture_size_x = Extent3d {
                width: config.height * 2,
                // For now use limited amount of path segments
                height: self.capacity,
                depth_or_array_layers: 1,
            };

            let solutions_texture_x = device.create_texture(&wgpu::TextureDescriptor {
                // All textures are stored as 3D, we represent our 2D texture
                // by setting depth to 1.
                size: texture_size_x,
                mip_level_count: 1, // We'll talk about this a little later
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // Most images are stored using sRGB so we need to reflect that here.
                format: wgpu::TextureFormat::Rgba32Float,
                // Used as output from the compute shader therefore STORAGE_BINDING
                // And used as input texture for the next pipeline
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                label: Some("solutions_texture_x"),
            });
            let solutions_texture_x_view =
                solutions_texture_x.create_view(&wgpu::TextureViewDescriptor::default());

            let area_directions_texture_x = device.create_texture(&wgpu::TextureDescriptor {
                // All textures are stored as 3D, we represent our 2D texture
                // by setting depth to 1.
                size: texture_size_x,
                mip_level_count: 1, // We'll talk about this a little later
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // Most images are stored using sRGB so we need to reflect that here.
                format: wgpu::TextureFormat::Rgba32Uint,
                // Used as output from the compute shader therefore STORAGE_BINDING
                // And used as input texture for the next pipeline
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                label: Some("gradient_texture_x"),
            });
            let area_directions_texture_x_view =
                area_directions_texture_x.create_view(&wgpu::TextureViewDescriptor::default());

            let input_texture_bind_group_x = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: input_texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&solutions_texture_x_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(
                            &area_directions_texture_x_view,
                        ),
                    },
                ],
                label: Some("input_texture_bind_group_x"),
            });

            let output_texture_bind_group_x =
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: output_texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&solutions_texture_x_view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(
                                &area_directions_texture_x_view,
                            ),
                        },
                    ],
                    label: Some("output_texture_bind_group_x"),
                });

            self.texture_size_x = texture_size_x;
            self.solutions_texture_x = solutions_texture_x;
            self.solutions_texture_x_view = solutions_texture_x_view;
            self.area_directions_texture_x = area_directions_texture_x;
            self.area_directions_texture_x_view = area_directions_texture_x_view;
            self.input_texture_bind_group_x = input_texture_bind_group_x;
            self.output_texture_bind_group_x = output_texture_bind_group_x;
        }
    }

    fn record_compute_x<'a>(&'a self, compute_pass: &mut wgpu::ComputePass<'a>) {
        if self.path_segment_count > 0 {
            compute_pass.set_bind_group(1, self.segments_bind_group.as_ref().unwrap(), &[]);
            compute_pass.set_bind_group(2, &self.output_texture_bind_group_x, &[]);
            compute_pass.dispatch_workgroups(self.path_segment_count as u32, 1, 1);
        }
    }

    fn record_compute_y<'a>(&'a self, compute_pass: &mut wgpu::ComputePass<'a>) {
        if self.path_segment_count > 0 {
            compute_pass.set_bind_group(1, self.segments_bind_group.as_ref().unwrap(), &[]);
            compute_pass.set_bind_group(2, &self.output_texture_bind_group_y, &[]);
            compute_pass.dispatch_workgroups(self.path_segment_count as u32, 1, 1);
        }
    }

    fn record<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.instance_count > 0 {
            // If instance_count > 0 then the instance buffer must exist
            debug_assert!(self.instance_buffer.is_some());
            render_pass.set_bind_group(1, self.segments_bind_group.as_ref().unwrap(), &[]);
            render_pass.set_bind_group(2, &self.input_texture_bind_group_x, &[]);
            render_pass.set_bind_group(3, &self.input_texture_bind_group_y, &[]);
            render_pass.set_vertex_buffer(0, self.instance_buffer.as_ref().unwrap().slice(..));
            render_pass.draw(0..6, 0..self.instance_count as _);
        }
    }
}

pub struct PathPipeline {
    config: wgpu::SurfaceConfiguration,
    compute_pipeline_x: wgpu::ComputePipeline,
    compute_pipeline_y: wgpu::ComputePipeline,
    pipeline: wgpu::RenderPipeline,
    globals_buffer: wgpu::Buffer,
    globals_bind_group: wgpu::BindGroup,

    segments_bind_group_layout: wgpu::BindGroupLayout,
    input_texture_bind_group_layout: wgpu::BindGroupLayout,
    output_texture_bind_group_layout: wgpu::BindGroupLayout,

    bundles: Vec<PathBundle>,
}
impl PathPipeline {
    const PATH_BUNDLE_SIZE: u32 = 2048;

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

        let input_texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: false },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: TextureSampleType::Uint,
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                ],
                label: Some("input_texture_bind_group_layout"),
            });

        let output_texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: StorageTextureAccess::WriteOnly,
                            format: TextureFormat::Rgba32Float,
                            view_dimension: TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: StorageTextureAccess::WriteOnly,
                            format: TextureFormat::Rgba32Uint,
                            view_dimension: TextureViewDimension::D2,
                        },
                        count: None,
                    },
                ],
                label: Some("output_texture_bind_group_layout"),
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
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
                // One for x
                &input_texture_bind_group_layout,
                // And one for y
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
                targets: &[Some(config.format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::R8Unorm,
                depth_write_enabled: false,
                depth_compare: CompareFunction::Never,
                stencil: StencilState {
                    front: StencilFaceState {
                        compare: CompareFunction::Always,
                        fail_op: StencilOperation::IncrementWrap,
                        depth_fail_op: Default::default(),
                        pass_op: Default::default(),
                    },
                    back: Default::default(),
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: Default::default(),
            }),
            multisample,
            multiview: None,
        });

        let compute_shader_x = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../../../../shader/path_compute_x_new.wgsl"
            ))),
        });

        let compute_shader_y = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../../../../shader/path_compute_y_new.wgsl"
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

        let compute_pipeline_x = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader_x,
            entry_point: "cs_main",
        });

        let compute_pipeline_y = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader_y,
            entry_point: "cs_main",
        });

        PathPipeline {
            config: config.clone(),
            compute_pipeline_x,
            compute_pipeline_y,
            pipeline,
            globals_buffer,
            globals_bind_group,
            segments_bind_group_layout,
            input_texture_bind_group_layout,
            output_texture_bind_group_layout,
            bundles: vec![],
        }
    }

    pub fn resize(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) {
        self.config = config.clone();
        let globals = primitive::Globals {
            width_height: util::pack(config.width as u16, config.height as u16),
            aspect_ratio: config.width as f32 / config.height as f32,
        };

        queue.write_buffer(&self.globals_buffer, 0, bytemuck::cast_slice(&[globals]));

        for bundle in &mut self.bundles {
            bundle.resize(
                device,
                queue,
                config,
                &self.input_texture_bind_group_layout,
                &self.output_texture_bind_group_layout,
            );
        }
    }

    pub fn record_compute<'a>(&'a self, compute_pass: &mut wgpu::ComputePass<'a>) {
        if self.bundles.len() > 0 {
            compute_pass.set_pipeline(&self.compute_pipeline_y);
            compute_pass.set_bind_group(0, &self.globals_bind_group, &[]);
            for bundle in &self.bundles {
                bundle.record_compute_y(compute_pass);
            }

            compute_pass.set_pipeline(&self.compute_pipeline_x);
            compute_pass.set_bind_group(0, &self.globals_bind_group, &[]);
            for bundle in &self.bundles {
                bundle.record_compute_x(compute_pass);
            }
        }
    }

    pub fn record<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.bundles.len() > 0 {
            // If instance_count > 0 then the instance buffer must exist

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.globals_bind_group, &[]);
            for bundle in &self.bundles {
                bundle.record(render_pass);
            }
        }
    }

    pub(crate) fn mount(&mut self, device: &wgpu::Device, paths: Vec<primitive::Path>) {
        self.bundles.clear();
        for chunk in paths.chunks(Self::PATH_BUNDLE_SIZE as usize) {
            let mut bundle = PathBundle::new(
                Self::PATH_BUNDLE_SIZE,
                device,
                &self.config,
                &self.input_texture_bind_group_layout,
                &self.output_texture_bind_group_layout,
            );
            bundle.mount(device, chunk.into(), &self.segments_bind_group_layout);
            self.bundles.push(bundle);
        }
    }
}
