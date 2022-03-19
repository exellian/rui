use std::collections::HashMap;
use std::hash::Hash;
use raw_window_handle::HasRawWindowHandle;
use wgpu::Backends;
use crate::node::Node;
use crate::renderer::wgpu::renderer_job::RenderJob;
use crate::surface::Surface;
use async_trait::async_trait;
use crate::renderer::wgpu::renderer_error::RendererError;
use crate::util;

pub struct RendererBase {
    instance: wgpu::Instance,
    device: wgpu::Device,
    adapter: wgpu::Adapter,
    queue: wgpu::Queue,
}
impl RendererBase {

    pub async fn new(instance: wgpu::Instance, surface: &wgpu::Surface) -> Result<Self, RendererError> {
        let adapter = match instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(surface),
            }).await {
            Some(a) => a,
            None => return Err(RendererError::AdapterNotFound)
        };
        let (device, queue) = match adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::all_webgpu_mask(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
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
    surfaces: HashMap<u64, wgpu::Surface>,
    jobs: HashMap<u64, RenderJob>,
}
impl Renderer {
    fn new() -> Self {
        Renderer {
            base: None,
            surfaces: HashMap::new(),
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
impl<E, T> crate::renderer::Renderer<E, T> for Renderer where
    E: 'static,
    T: 'static + Surface<E> + Hash + HasRawWindowHandle + Send + Sync
{
    type Error = RendererError;

    //TODO split up in parts and functions that make sense
    async fn mount(&mut self, surface: &T, node: &Node) -> Result<(), Self::Error> {
        let sid = util::id(surface);
        let surface = match self.surfaces.get(&sid) {
            None => {
                // Dynamically creating render base for the first surface that gets mounted
                let surface_handle = match &mut self.base {
                    Some(base) => {
                        unsafe {
                            base.instance.create_surface(surface)
                        }
                    },
                    rb@None => {
                        let instance = wgpu::Instance::new(Backends::all());
                        // Creating a surface first before creating the base, so that
                        // the base can use it to find a suitable adapter (GPU)
                        let first_surface = unsafe {
                            instance.create_surface(surface)
                        };
                        *rb = Some(RendererBase::new(instance, &first_surface).await?);
                        first_surface
                    }
                };
                let size = surface.inner_size();
                let base = self.base.as_ref().unwrap();
                let swapchain_format = surface_handle.get_preferred_format(&base.adapter).unwrap();
                let mut config = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: swapchain_format,
                    width: size.width,
                    height: size.height,
                    present_mode: wgpu::PresentMode::Fifo, //TODO add option for disabling vsync
                };
                surface_handle.configure(&base.device, &config);
                self.surfaces.insert(sid, surface_handle);
                self.surfaces.get(&sid).unwrap()
            }
            Some(sh) => sh
        };

        Ok(())
    }
}