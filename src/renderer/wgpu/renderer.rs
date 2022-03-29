use std::collections::HashMap;
use raw_window_handle::HasRawWindowHandle;
use crate::node::Node;
use crate::renderer::wgpu::pipeline::renderer_job::RenderJob;
use crate::surface::{SurfaceAdapter, SurfaceId};
use async_trait::async_trait;
use crate::renderer::wgpu::renderer_error::RendererError;
use crate::Backend;
use crate::util::Extent;

pub struct RendererBase {
    pub(crate) instance: wgpu::Instance,
    pub(crate) device: wgpu::Device,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) queue: wgpu::Queue
}
impl RendererBase {

    pub async fn new(backend_bits: wgpu_types::Backends, instance: wgpu::Instance, surface: &wgpu::Surface) -> Result<Self, RendererError> {

        let adapter = match wgpu::util::initialize_adapter_from_env_or_default(&instance, backend_bits, Some(surface)).await {
            Some(a) => a,
            None => return Err(RendererError::AdapterNotFound)
        };
        let optional_features = wgpu_types::Features::POLYGON_MODE_LINE;
        let required_features = wgpu::Features::empty();
        let adapter_features = adapter.features();
        // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the surface.
        let needed_limits = wgpu::Limits::downlevel_webgl2_defaults()
            .using_resolution(adapter.limits());

        let (device, queue) = match adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: (optional_features & adapter_features) | required_features,
                    limits: needed_limits,
                },
                None,
            ).await {
            Ok(dq) => dq,
            Err(e) => return Err(RendererError::DeviceCreationFailed(e))
        };

        Ok(RendererBase {
            instance,
            device,
            adapter,
            queue
        })
    }
}
pub struct Renderer {
    base: Option<RendererBase>,
    jobs: HashMap<SurfaceId, RenderJob>
}
impl Renderer {
    fn new() -> Self {
        Renderer {
            base: None,
            jobs: HashMap::new()
        }
    }
}
impl Default for Renderer {
    fn default() -> Self {
        Renderer::new()
    }
}

#[async_trait]
impl<B> crate::renderer::Renderer<B> for Renderer where
    B: Backend,
    B::Surface: HasRawWindowHandle,
{
    type Error = RendererError;

    //TODO split up in parts and functions that make sense
    async fn mount(&mut self, surface: &B::Surface, node: &Node) -> Result<(), Self::Error> {
        let sid = surface.id();
        let (job, base) = match self.jobs.get_mut(&sid) {
            None => {
                // Dynamically creating render base for the first surface that gets mounted
                let surface_handle = match &mut self.base {
                    Some(base) => {
                        unsafe {
                            base.instance.create_surface(surface)
                        }
                    },
                    rb@None => {
                        let backend_bits = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
                        let instance = wgpu::Instance::new(backend_bits);

                        // Creating a surface first before creating the base, so that
                        // the base can use it to find a suitable adapter (GPU)
                        let first_surface = unsafe {
                            instance.create_surface(surface)
                        };
                        *rb = Some(RendererBase::new(backend_bits, instance, &first_surface).await?);
                        first_surface
                    }
                };
                let size = surface.inner_size();
                let base = self.base.as_ref().unwrap();
                let swapchain_format = surface_handle.get_preferred_format(&base.adapter).unwrap();
                let config = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: swapchain_format,
                    width: size.width,
                    height: size.height,
                    present_mode: wgpu::PresentMode::Fifo, //TODO add option for disabling vsync
                };
                surface_handle.configure(&base.device, &config);
                self.jobs.insert(sid, RenderJob::new(
                    &base.device,
                    config,
                    surface_handle
                ));
                (self.jobs.get_mut(&sid).unwrap(), base)
            }
            Some(sh) => (sh, self.base.as_ref().unwrap())
        };
        //Creation of rendering objects
        job.mount(&base.device, node).await;
        Ok(())
    }

    async fn resize(&mut self, surface_id: SurfaceId, size: Extent) -> Result<(), Self::Error> {
        let c = self.jobs.get_mut(&surface_id).unwrap();
        let base = self.base.as_mut().unwrap();
        c.resize(&base.device, &base.queue, size);
        Ok(())
    }

    fn render(&self, surface_id: SurfaceId) -> Result<(), Self::Error> {
        let base = self.base.as_ref()
            .expect("Can't render with no surface mounted!");
        let job = self.jobs.get(&surface_id)
            .expect("Invalid surface id!");
        let frame = job.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame.texture
            .create_view(&wgpu::TextureViewDescriptor::default());


        let mut encoder = base.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: true,
                        },
                    }
                ],
                depth_stencil_attachment: None,
            });
            job.record(&mut render_pass);
        }

        base.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}