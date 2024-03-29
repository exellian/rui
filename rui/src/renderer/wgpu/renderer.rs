use std::collections::HashMap;
use wgpu::{LoadOp, Operations, RenderPassDepthStencilAttachment};
use wgpu_types::{Backends, Color, CompositeAlphaMode};

use rui_io::surface::SurfaceId;
use rui_util::{be, bs, Extent};

use crate::node::Node;
use crate::renderer::wgpu::pipeline::renderer_job::RenderJob;
use crate::renderer::wgpu::RendererError;
use crate::renderer::MSAA;
use crate::Backend;

pub struct RendererBase {
    pub(crate) instance: wgpu::Instance,
    pub(crate) device: wgpu::Device,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) queue: wgpu::Queue,
}
impl RendererBase {
    pub async fn new(
        backend_bits: wgpu_types::Backends,
        instance: wgpu::Instance,
        surface: &wgpu::Surface,
    ) -> Result<Self, RendererError> {
        let adapter = match wgpu::util::initialize_adapter_from_env_or_default(
            &instance,
            backend_bits,
            Some(surface),
        )
        .await
        {
            Some(a) => a,
            None => return Err(RendererError::AdapterNotFound),
        };
        let optional_features = wgpu_types::Features::POLYGON_MODE_LINE;
        let required_features = wgpu::Features::empty();
        let adapter_features = adapter.features();
        // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the surface.
        let mut needed_limits =
            wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits());
        /*
                needed_limits.max_storage_buffers_per_shader_stage = 4;
                needed_limits.max_storage_buffer_binding_size = 128 << 20;
                needed_limits.max_storage_textures_per_shader_stage = 4;
                needed_limits.max_compute_workgroup_size_x = 128;
                needed_limits.max_compute_workgroup_size_y = 1;
                needed_limits.max_compute_workgroup_size_z = 1;
        */
        let (device, queue) = match adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: (optional_features & adapter_features) | required_features,
                    limits: needed_limits,
                },
                None,
            )
            .await
        {
            Ok(dq) => dq,
            Err(e) => return Err(RendererError::DeviceCreationFailed(e)),
        };

        Ok(RendererBase {
            instance,
            device,
            adapter,
            queue,
        })
    }
}
pub struct Renderer<B>
where
    B: Backend,
{
    base: Option<RendererBase>,
    jobs: HashMap<SurfaceId, RenderJob<B>>,
}
impl<B> Renderer<B>
where
    B: Backend,
{
    fn new() -> Self {
        Renderer {
            base: None,
            jobs: HashMap::new(),
        }
    }
}
impl<B> Default for Renderer<B>
where
    B: Backend,
{
    fn default() -> Self {
        Renderer::new()
    }
}

impl<B> crate::renderer::Renderer<B> for Renderer<B>
where
    B: Backend,
{
    type Error = RendererError;

    //TODO split up in parts and functions that make sense
    fn mount(
        &mut self,
        surface: &rui_io::surface::Surface,
        node: &mut Node,
    ) -> Result<(), Self::Error> {
        let sid = surface.id();
        let (job, base) = match self.jobs.get_mut(&sid) {
            None => {
                // Dynamically creating render base for the first surface that gets mounted
                let surface_handle = match &mut self.base {
                    Some(base) => unsafe { base.instance.create_surface(surface) },
                    rb @ None => {
                        //let backend_bits =
                        //    wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
                        let backend_bits = Backends::VULKAN;
                        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);

                        // Creating a surface first before creating the base, so that
                        // the base can use it to find a suitable adapter (GPU)
                        let first_surface = unsafe { instance.create_surface(surface) };
                        *rb = Some(pollster::block_on(RendererBase::new(
                            backend_bits,
                            instance,
                            &first_surface,
                        ))?);
                        first_surface
                    }
                };
                let size = surface.inner_size();
                let base = self.base.as_ref().unwrap();
                let swapchain_format = surface_handle.get_supported_formats(&base.adapter)[0];
                let config = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: swapchain_format,
                    width: size.width,
                    height: size.height,
                    present_mode: wgpu::PresentMode::Fifo, //TODO add option for disabling vsync
                    alpha_mode: CompositeAlphaMode::Auto,
                };
                surface_handle.configure(&base.device, &config);
                self.jobs.insert(
                    sid,
                    RenderJob::new(&base.device, config, surface_handle, MSAA::X4),
                );
                (self.jobs.get_mut(&sid).unwrap(), base)
            }
            Some(sh) => (sh, self.base.as_ref().unwrap()),
        };
        //Creation of rendering objects

        pollster::block_on(job.mount(&base.device, &base.queue, node));
        Ok(())
    }

    fn resize(
        &mut self,
        surface: &rui_io::surface::Surface,
        size: Extent,
    ) -> Result<(), Self::Error> {
        bs!(resize);
        let job = self.jobs.get_mut(&surface.id()).unwrap();
        let base = self.base.as_mut().unwrap();
        job.resize(&base.device, &base.queue, size);
        be!(resize);
        Ok(())
    }

    fn render(&mut self, surface: &rui_io::surface::Surface) -> Result<(), Self::Error> {
        bs!(render_time);
        let base = self
            .base
            .as_ref()
            .expect("Can't render with no surface mounted!");
        let job = self.jobs.get(&surface.id()).expect("Invalid surface id!");
        let frame = match job.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(_) => {
                job.surface.configure(&base.device, &job.config);
                job.surface
                    .get_current_texture()
                    .expect("Failed to acquire next surface texture!")
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let render_pass_color_attachment_pre_pass = match job.msaa {
            MSAA::X1 => wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            },
            _ => wgpu::RenderPassColorAttachment {
                view: job.multisampling_framebuffer.as_ref().unwrap(),
                resolve_target: Some(&view),
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            },
        };

        let render_pass_color_attachment = match job.msaa {
            MSAA::X1 => wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::WHITE),
                    store: true,
                },
            },
            _ => wgpu::RenderPassColorAttachment {
                view: job.multisampling_framebuffer.as_ref().unwrap(),
                resolve_target: Some(&view),
                ops: Operations {
                    load: LoadOp::Clear(Color::WHITE),
                    store: true,
                },
            },
        };

        let mut encoder = base
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            /*let mut render_pass_compute =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            job.record_compute(&mut render_pass_compute);
             */
            let mut pre_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Stencil pre pass"),
                // We don't need color attachments in this pass
                color_attachments: &[],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &job.stencil_framebuffer,
                    depth_ops: None,
                    stencil_ops: Some(Operations {
                        load: LoadOp::Clear(0),
                        store: true,
                    }),
                }),
            });
            job.record_prepass(&mut pre_pass);
        }
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(render_pass_color_attachment)],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &job.stencil_framebuffer,
                    depth_ops: None,
                    stencil_ops: Some(Operations {
                        load: LoadOp::Load,
                        store: false,
                    }),
                }),
            });
            job.record(&mut render_pass);
        }

        base.queue.submit(Some(encoder.finish()));
        frame.present();
        be!(render_time);
        Ok(())
    }

    fn request_render(&self) -> Result<(), Self::Error> {
        for (_, _job) in &self.jobs {
            //job.surface_adapter.request_redraw();
        }
        Ok(())
    }
}
