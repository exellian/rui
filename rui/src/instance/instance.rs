use std::collections::{BTreeMap, HashMap, HashSet};
use std::future::Future;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use raw_window_handle::HasRawWindowHandle;

use rui_async::{Scheduler, Status};
use rui_io::event::{Event, Flow, MainEventLoop};
use rui_io::surface::{SurfaceEvent, SurfaceId};
use rui_util::alloc::mpsc;

use crate::instance::error::Error;
use crate::instance::main_loop_request::MainLoopRequest;
use crate::instance::InstanceShared;
use crate::renderer::Renderer;
use crate::surface::SurfaceSharedState;
use crate::{Backend, Node};

pub struct Instance<B>
where
    B: Backend,
{
    pub(crate) renderer: B::Renderer,
    nodes: BTreeMap<SurfaceId, Node>,
    // For now just create everything on the main thread
    main_loop_receiver: mpsc::Receiver<MainLoopRequest>,
}
//TODO check thread safety for Instance struct
unsafe impl<B> Send for Instance<B> where B: Backend {}
//TODO check thread safety for Instance struct
unsafe impl<B> Sync for Instance<B> where B: Backend {}

impl<B> Instance<B>
where
    B: Backend + 'static,
{
    pub fn new(renderer: B::Renderer) -> (Self, InstanceShared) {
        let (main_loop_sender, main_loop_receiver) = mpsc::unbounded();
        (
            Instance {
                renderer,
                nodes: BTreeMap::new(),
                main_loop_receiver,
            },
            InstanceShared::new(main_loop_sender),
        )
    }

    pub fn mount(
        &mut self,
        surface: &rui_io::surface::Surface,
        mut node: Node,
    ) -> Result<(), Error<B>> {
        if let Err(err) = self.renderer.mount(surface, &mut node) {
            return Err(Error::RendererError(err));
        }
        self.nodes.insert(surface.id(), node);
        Ok(())
    }
    /*
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Init => {}
            Event::SurfaceEvent { id, event } => match event {
                SurfaceEvent::Resized(extent) => {
                    self.renderer.resize(id, extent).unwrap();
                    //self.renderer.render(id).unwrap();
                }
                SurfaceEvent::Redraw => {
                    self.renderer.render(id).unwrap();
                }
                _ => {}
            },
            Event::EventsCleared => {
                self.renderer.request_render();
            }
            Event::Default => {}
        }
    }*/

    /// Returns nothing and doesn't terminate -> therefore the
    /// return type is `!`
    ///
    /// This function starts all necessary threads to run the application.
    ///
    /// # Arguments
    /// * `self` - takes self
    pub fn run(mut self, start_app: impl Future<Output = ()>) -> ! {
        let mut start_app = Some(start_app);
        let mut main_event_loop = MainEventLoop::new();
        let scheduler = Scheduler::new();
        let mut main_worker = scheduler.new_worker();

        let mut surfaces = HashMap::new();
        let mut mounted = HashSet::new();

        main_event_loop.run(move |target, event, flow| {
            *flow = Flow::Wait;
            if let Some(Event::Init) = event {
                // Start the app
                main_worker.spawn(start_app.take().unwrap());
            }
            if let Some(Event::SurfaceEvent { id, event }) = event {
                match event {
                    SurfaceEvent::Resized(extent) => match surfaces.get(id) {
                        Some((surface, _)) => {
                            if mounted.contains(id) {
                                self.renderer.resize(surface, extent.clone()).unwrap();
                                self.renderer.render(surface).unwrap();
                            }
                        }
                        None => {}
                    },
                    SurfaceEvent::Redraw => {
                        if mounted.contains(id) {
                            let (surface, _) = surfaces.get(id).unwrap();
                            self.renderer.render(surface).unwrap();
                        }
                    }
                    SurfaceEvent::ShouldClose => {}
                }
            }
            match self.main_loop_receiver.try_recv() {
                None => {}
                Some(req) => match req {
                    MainLoopRequest::CreateSurface { attr, sender } => {
                        let surface = rui_io::surface::Surface::new(target, &attr);
                        let surface_shared_state = Arc::new(RwLock::new(SurfaceSharedState::new(
                            surface.id(),
                            attr,
                            surface.raw_window_handle(),
                        )));
                        surfaces.insert(surface.id(), (surface, surface_shared_state.clone()));
                        sender.send(surface_shared_state);
                    }
                    MainLoopRequest::MountNode {
                        surface_id,
                        node,
                        sender,
                    } => match surfaces.get(&surface_id) {
                        None => sender.send(Err(crate::error::Error::MountError)),
                        Some((surface, _)) => {
                            self.mount(surface, node).unwrap();
                            mounted.insert(surface.id());
                            self.renderer.render(surface).unwrap();
                            sender.send(Ok(()))
                        }
                    },
                },
            }

            // Change the worker flow to poll if we still have tasks to poll
            // and change to waiting when work is not present
            match main_worker.poll() {
                Status::Pending => *flow = Flow::Poll,
                Status::Ready => *flow = Flow::Wait,
            }
        })
    }
}
