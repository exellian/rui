use std::collections::{BTreeMap};
use std::sync::{Arc, mpsc};

use tokio::sync::Mutex;
use crate::event::{Event, EventLoop};
use crate::{Backend, Node};
use crate::instance::backend::WGpu;
use crate::instance::error::Error;
use crate::instance::main_thread::{MainThreadRequest};
use crate::renderer::Renderer;
use crate::surface::{SurfaceAdapter, SurfaceAttributes, SurfaceEvent, SurfaceFactory, SurfaceId};

pub struct Instance<B> where
    B: Backend
{
    /// Event loop of the main thread.
    main_thread_event_loop: Option<B::EventLoop>,
    pub(crate) renderer: B::Renderer,
    nodes: BTreeMap<SurfaceId, Node>,
    main_thread_sender: Option<mpsc::Sender<MainThreadRequest<B>>>,
}
//TODO check thread safety for Instance struct
unsafe impl<B> Send for Instance<B> where B: Backend {}
//TODO check thread safety for Instance struct
unsafe impl<B> Sync for Instance<B> where B: Backend {}

impl Default for Instance<WGpu> {
    fn default() -> Self {
        let event_loop = crate::event::winit::EventLoop::default();
        let renderer = crate::renderer::wgpu::Renderer::default();
        Instance::new(event_loop, renderer)
    }
}
impl<B> Instance<B> where
    B: Backend + 'static
{

    pub fn new(event_loop: B::EventLoop, renderer: B::Renderer) -> Self {
        Instance {
            main_thread_event_loop: Some(event_loop),
            renderer,
            nodes: BTreeMap::new(),
            main_thread_sender: None
        }
    }

    pub(crate) async fn _mount(&mut self, surface: Arc<B::Surface>, mut node: Node) -> Result<(), Error<B>> {
        if let Err(err) = self.renderer.mount(surface.clone(), &mut node).await {
            return Err(Error::RendererError(err))
        }
        self.nodes.insert(surface.id(), node);
        Ok(())
    }

    pub fn mount(&mut self, surface: impl Into<Arc<B::Surface>>, node: Node) -> Result<(), Error<B>>{
        pollster::block_on(self._mount(surface.into(), node))
    }

    pub(crate) fn create_surface(&self, attributes: SurfaceAttributes) -> Result<B::Surface, <B::SurfaceFactory as SurfaceFactory>::Error> {
        match &self.main_thread_event_loop {
            // In this case we are on another thread than the main thread.
            // The run method has been called and we don't have a reference to the
            // EventLoopTarget. Therefore we have to create the surface using channels.
            None => {
                let (sender, r) = mpsc::channel();

                self.main_thread_sender.as_ref().unwrap().send(MainThreadRequest::CreateSurface {
                    attributes,
                    sender
                }).unwrap();
                r.recv().unwrap()
            }
            // In this case the instance is not started up yet.
            // Therefore we have a reference to the event loop and can
            // start create the surface directly
            Some(event_loop) => {
                B::SurfaceFactory::build(event_loop, attributes)
            }
        }

    }

    fn main_thread_handle(receiver: &mut mpsc::Receiver<MainThreadRequest<B>>, event_loop: &<B::EventLoop as EventLoop<B::UserEvent>>::EventLoopTarget) {

        if let Ok(req) = receiver.try_recv() {
            match req {
                MainThreadRequest::CreateSurface { attributes, sender } => {
                    let surface_result = B::SurfaceFactory::build(event_loop, attributes);
                    if let Err(_) = sender.send(surface_result) {
                        panic!("Async runtime hang up!")
                    }
                }
            }
        }
    }

    async fn handle_mt(&mut self, event: Event<B::UserEvent>) {
        match event {
            Event::UserEvent(event) => {

            }
            Event::SurfaceEvent { id, event } => match event {
                SurfaceEvent::Resized(extent) => {
                    self.renderer.resize( id, extent).await.unwrap();
                    //self.renderer.render(id).unwrap();
                }
                SurfaceEvent::Redraw => {
                    self.renderer.render(id).unwrap();
                }
            }
            Event::EventsCleared => {
                self.renderer.request_render();
            }
        }
    }

    /// Returns nothing and doesn't terminate -> therefore the
    /// return type is `!`
    ///
    /// This function starts all necessary threads to run the application.
    /// It must be called from the main thread to work on all platforms!
    ///
    /// # Arguments
    /// * `self` - This function takes ownership of self because it doesn't terminate
    pub fn run(mut self) -> ! {
        // Split event loop from instance
        let main_thread_event_loop = self.main_thread_event_loop.take().unwrap();
        // Currently only supporting multithreaded runtime
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create async runtime!");

        let (main_thread_sender, mut main_thread_receiver) = mpsc::channel();
        self.main_thread_sender = Some(main_thread_sender);

        let this = Arc::new(Mutex::new(self));
        main_thread_event_loop.run(move |event, event_loop_target| {
            let this = this.clone();

            // Handle messages on the main thread
            // e.g: Window creation
            Self::main_thread_handle(&mut main_thread_receiver, event_loop_target);

            // Handle event on the multithreaded async runtime
            runtime.spawn(async move {
                let mut guard = this.lock().await;
                guard.handle_mt(event).await;
            });
        });
        loop {}
    }
}