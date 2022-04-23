use std::borrow::Cow;
use std::fs::File;
use std::io::Read;
use image::load;
use wgpu::BindGroupLayout;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu_types::BufferUsages;
use crate::renderer::wgpu::primitive;
use crate::util::Resource;

pub struct Texture {
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    instance_buffer: wgpu::Buffer,
    instance_bind_group: wgpu::BindGroup,
    texture_bind_group: wgpu::BindGroup
}

pub struct ImagePipeline {
    pipeline: wgpu::RenderPipeline,
    textures: Vec<Texture>,
    sampler: wgpu::Sampler,
    globals_buffer: wgpu::Buffer,
    globals_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: BindGroupLayout,
    instance_bind_group_layout: BindGroupLayout
}
impl ImagePipeline {

    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let globals = primitive::Globals {
            aspect_ratio: config.width as f32 / config.height as f32
        };

        let globals_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("globals_buffer"),
            contents: bytemuck::cast_slice(&[globals]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });

        let globals_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("globals_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ]
        });

        let globals_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &globals_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: globals_buffer.as_entire_binding(),
                }
            ],
            label: Some("globals_bind_group"),
        });

        let instance_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("instance_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ]
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../../../../shader/image.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            // Group 0 globals, 1 instance, 2 texture
            bind_group_layouts: &[&globals_bind_group_layout, &instance_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
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

        ImagePipeline {
            pipeline,
            textures: vec![],
            sampler,
            globals_buffer,
            globals_bind_group,
            texture_bind_group_layout,
            instance_bind_group_layout
        }
    }

    pub fn record<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);

        for instance in &self.textures {
            render_pass.set_bind_group(0, &self.globals_bind_group, &[]);
            render_pass.set_bind_group(1, &instance.instance_bind_group, &[]);
            render_pass.set_bind_group(2, &instance.texture_bind_group, &[]);
            render_pass.draw(0..6, 0..1);
        }
    }

    async fn load(resource: &Resource) -> Vec<u8> {
        match resource {
            Resource::Path(path) => {
                let mut file = File::open(path).unwrap();
                let mut contents = vec![];
                file.read_to_end(&mut contents).unwrap();
                contents
            }
        }
    }

    pub fn resize(&self, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration) {
        let globals = primitive::Globals {
            aspect_ratio: config.width as f32 / config.height as f32
        };
        queue.write_buffer(&self.globals_buffer, 0, bytemuck::cast_slice(&[globals]));
    }

    pub async fn mount(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, images: &Vec<primitive::Image>) {
        let mut textures = Vec::with_capacity(images.len());
        for i in images {

            let bytes = Self::load(&i.resource).await;
            let image = image::load_from_memory(&bytes).unwrap();
            let rgba = image.to_rgba8();
            let dimensions = rgba.dimensions();

            let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: Some("instance_buffer"),
                contents: bytemuck::cast_slice(&[i.instance]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
            });

            let texture_size = wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            };
            let texture = device.create_texture(
                &wgpu::TextureDescriptor {
                    // All textures are stored as 3D, we represent our 2D texture
                    // by setting depth to 1.
                    size: texture_size,
                    mip_level_count: 1, // We'll talk about this a little later
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    // Most images are stored using sRGB so we need to reflect that here.
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
                    // COPY_DST means that we want to copy data to this texture
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    label: Some("texture"),
                }
            );
            queue.write_texture(
                // Tells wgpu where to copy the pixel data
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                // The actual pixel data
                rgba.as_raw(),
                // The layout of the texture
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                    rows_per_image: std::num::NonZeroU32::new(dimensions.1),
                },
                texture_size,
            );

            // We don't need to configure the texture view much, so let's
            // let wgpu define it.
            let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

            let texture_bind_group = device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    layout: &self.texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&texture_view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&self.sampler),
                        }
                    ],
                    label: Some("diffuse_bind_group"),
                }
            );

            let instance_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &self.instance_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: instance_buffer.as_entire_binding(),
                    }
                ],
                label: Some("camera_bind_group"),
            });

            textures.push(Texture {
                texture,
                texture_view,
                instance_buffer,
                instance_bind_group,
                texture_bind_group
            });
        }
        self.textures = textures;
    }
}