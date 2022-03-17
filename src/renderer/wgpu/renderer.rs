use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use raw_window_handle::HasRawWindowHandle;
use wgpu::Backends;
use crate::node::Node;
use crate::renderer::wgpu::renderer_job::RenderJob;
use crate::surface::Surface;
use async_trait::async_trait;
use crate::renderer::wgpu::renderer_error::RendererError;
use crate::renderer::wgpu::surface_handle::SurfaceHandle;

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

    fn surface_id<T>(h: &T) -> u64 where T: Hash {
        let mut hasher = DefaultHasher::new();
        h.hash(&mut hasher);
        hasher.finish()
    }
}

#[async_trait]
impl<T> crate::renderer::Renderer<T> for Renderer
    where T: 'static + Surface + Hash + HasRawWindowHandle + Send + Sync
{
    type Error = RendererError;
    type SurfaceHandle = SurfaceHandle;

    async fn mount_surface(&mut self, surface: &T) -> Result<Self::SurfaceHandle, Self::Error> {
        let sid = Renderer::surface_id(surface);
        if self.surfaces.contains_key(&sid) {
            return Ok(sid);
        }
        // Dynamically creating render base for first surface that gets mounted
        let s = match &mut self.base {
            Some(base) => unsafe {
                base.instance.create_surface(surface)
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
        self.surfaces.insert(sid, s);
        Ok(sid)
    }

    async fn mount_component(&mut self, surface: Self::SurfaceHandle, component: &Node) -> Result<(), Self::Error> {
        todo!()
    }
}