use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::hash::{Hasher};
use std::sync::{Arc};
use tokio::sync::{mpsc, RwLock};
use crate::event::{Event, EventLoop};
use crate::{Backend, Node};
use crate::instance::backend::WGpu;
use crate::instance::error::Error;
use crate::instance::main_thread::{MainThreadRequest, MainThreadResponse};
use crate::util::{Extent, Sender};

pub struct Instance<B> where
    B: Backend
{
    event_loop: Option<B::EventLoop>,
    sender: std::sync::mpsc::Sender<MainThreadRequest>,
    receiver: mpsc::UnboundedReceiver<MainThreadResponse<B>>,
    pub(crate) renderer: Arc<RwLock<B::Renderer>>,
    nodes: BTreeMap<u64, Node>,
}
//TODO check thread safety for Instance struct
unsafe impl<B> Send for Instance<B> where B: Backend {}

impl<T> Default for Instance<WGpu<T>> where T: 'static + Sync + Send + Debug {
    fn default() -> Self {
        let event_loop = crate::event::winit::EventLoop::default();
        let renderer = crate::renderer::wgpu::Renderer::default();
        Instance::new(event_loop, renderer)
    }
}
impl<B> Instance<B> where
    B: Backend
{

    pub fn new(event_loop: B::EventLoop, renderer: B::Renderer) -> Self {

        Instance {
            event_loop: Some(event_loop),
            sender: ,
            receiver: (),
            renderer: Arc::new(RwLock::new(renderer)),
            nodes: BTreeMap::new()
        }
    }

    pub(crate) async fn mount(&mut self, surface: &B::Surface, node: Node) -> Result<(), Error<B>>{
        /*
        let sid = util::id(surface);
        if let Err(err) = self.renderer.mount(surface, &node).await {
            return Err(Error::RendererError(err))
        }
        self.nodes.insert(sid, node);

         */
        Ok(())
    }

    // Method is called from different threads but should execute on the main thread
    pub fn create_surface(&self, title: &str, size: Extent) {

    }

    async fn handle_event(&mut self, event: Event<B::UserEvent>) {
        match event {
            Event::UserEvent(event) => {

            }
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
        let event_loop = self.event_loop.take().unwrap();
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to creat asynchronous Runtime! Aborting.");
        runtime.block_on(async move {
            while let Some(event) = receiver.recv().await {
                self.handle_event(event).await;
            }
        });
        event_loop.run(Sender::new(sender));
    }
}