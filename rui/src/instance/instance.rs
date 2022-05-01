use std::collections::{BTreeMap};
use std::sync::Arc;

use rui_io::event::Event;
use rui_io::surface::{SurfaceEvent, SurfaceId};
use crate::{Backend, Node};
use crate::instance::backend::WGpu;
use crate::instance::error::Error;
use crate::renderer::Renderer;
use crate::surface::Surface;

pub struct Instance<B> where
    B: Backend
{
    pub(crate) renderer: B::Renderer,
    nodes: BTreeMap<SurfaceId, Node>,
    pub(crate) runtime: Runtime
}
//TODO check thread safety for Instance struct
unsafe impl<B> Send for Instance<B> where B: Backend {}
//TODO check thread safety for Instance struct
unsafe impl<B> Sync for Instance<B> where B: Backend {}

impl Default for Instance<WGpu> {
    fn default() -> Self {
        let renderer = crate::renderer::wgpu::Renderer::default();
        Instance::new(renderer)
    }
}
impl<B> Instance<B> where
    B: Backend + 'static
{

    pub fn new(renderer: B::Renderer) -> Self {
        Instance {
            renderer,
            nodes: BTreeMap::new(),
            runtime: Runtime::new(4)
        }
    }

    pub(crate) async fn _mount(&mut self, surface: Arc<Surface>, mut node: Node) -> Result<(), Error<B>> {
        if let Err(err) = self.renderer.mount(surface.clone(), &mut node).await {
            return Err(Error::RendererError(err))
        }
        self.nodes.insert(surface.id(), node);
        Ok(())
    }

    pub fn mount(&mut self, surface: impl Into<Arc<Surface>>, node: Node) -> Result<(), Error<B>> {
        pollster::block_on(self._mount(surface.into(), node))
    }

    async fn handle_event(&mut self, event: Event) {
        match event {
            Event::SurfaceEvent { id, event } => match event {
                SurfaceEvent::Resized(extent) => {
                    self.renderer.resize( id, extent).await.unwrap();
                    //self.renderer.render(id).unwrap();
                }
                SurfaceEvent::Redraw => {
                    self.renderer.render(id).unwrap();
                }
                _ => {}
            }
            Event::EventsCleared => {
                self.renderer.request_render();
            },
            Event::Default => {}
        }
    }

    /// Returns nothing and doesn't terminate -> therefore the
    /// return type is `!`
    ///
    /// This function starts all necessary threads to run the application.
    ///
    /// # Arguments
    /// * `self` - This function takes ownership of self because it doesn't terminate
    pub fn run(mut self) -> ! {

        let this = Arc::new(self);
        this.clone().runtime.run(async move {
            loop {
                let event = this.runtime.recv_event().await;
            }
        })
    }
}