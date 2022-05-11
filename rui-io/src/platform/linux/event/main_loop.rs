use crate::event::inner::{InnerFlow, InnerLoop};
use crate::event::Event;
use crate::surface::SurfaceId;
use rui_util::Extent;
use smithay_client_toolkit::environment::Environment;
use smithay_client_toolkit::reexports::client::Display;
use smithay_client_toolkit::shell::Shell;
use smithay_client_toolkit::window::{FallbackFrame, Window};
use smithay_client_toolkit::{default_environment, new_default_environment};
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;
use std::sync::Arc;
use wayland_client::EventQueue;

default_environment!(MyApp, desktop);

pub enum NextAction {
    None,
    Refresh,
    Redraw,
    Resize,
    Close,
}

pub struct WindowStateShared {
    next_action: NextAction,
    drawen_once: bool,
    size: Extent,
}
impl WindowStateShared {
    pub fn new(size: Extent) -> Self {
        WindowStateShared {
            next_action: NextAction::None,
            drawen_once: false,
            size,
        }
    }

    pub fn set_size(&mut self, size: Extent) {
        self.size = size;
    }

    pub fn get_size(&self) -> Extent {
        self.size
    }

    pub fn is_drawen_once(&mut self) -> bool {
        self.drawen_once
    }

    pub fn signal_should_close(&mut self) {
        self.next_action = NextAction::Close
    }

    pub fn signal_drawen_once(&mut self) {
        self.drawen_once = true;
    }

    pub fn signal_should_redraw(&mut self) {
        self.next_action = NextAction::Redraw
    }

    pub fn signal_should_refresh(&mut self) {
        self.next_action = NextAction::Refresh
    }

    pub fn signal_should_resize(&mut self) {
        self.next_action = NextAction::Resize
    }

    pub fn take_next_action(&mut self) -> NextAction {
        let mut next = NextAction::None;
        mem::swap(&mut next, &mut self.next_action);
        next
    }
}

pub struct WindowState {
    pub(crate) window: Window<FallbackFrame>,
    shared: Arc<RefCell<WindowStateShared>>,
}

impl WindowState {
    pub fn new(window: Window<FallbackFrame>, shared: Arc<RefCell<WindowStateShared>>) -> Self {
        WindowState { window, shared }
    }
}

pub struct MainLoop {
    pub(crate) wl_display: Display,
    main_event_queue: EventQueue,
    pub(crate) windows: HashMap<SurfaceId, WindowState>,
    environment: Environment<MyApp>,
    callback: Option<Rc<RefCell<dyn FnMut(&Event)>>>,
}

#[cfg(debug_assertions)]
fn debug_printout(environment: &Environment<MyApp>) {
    println!("== Smithay's compositor info tool ==\n");

    // print the best supported shell
    println!(
        "-> Most recent shell supported by the compositor is {}.",
        match environment.get_shell() {
            Some(Shell::Wl(_)) => "the legacy wl_shell",
            Some(Shell::Zxdg(_)) => "the old unstable xdg_shell (zxdg_shell_v6)",
            Some(Shell::Xdg(_)) => "the current xdg_shell",
            None => "nothing",
        }
    );
    println!();

    // print the outputs
    let outputs = environment.get_all_outputs();
    println!("-> Compositor advertised {} outputs:", outputs.len());
    for output in outputs {
        smithay_client_toolkit::output::with_output_info(&output, |info| {
            println!(
                "  -> #{}: {} ({}), with scale factor of {}",
                info.id, info.model, info.make, info.scale_factor
            );
            println!("     Possible modes are:");
            for mode in &info.modes {
                println!(
                    "     -> [{}{}] {} x {} @ {}.{} Hz",
                    if mode.is_preferred { "p" } else { " " },
                    if mode.is_current { "c" } else { " " },
                    mode.dimensions.0,
                    mode.dimensions.1,
                    mode.refresh_rate / 1000,
                    mode.refresh_rate % 1000
                );
            }
        });
    }
    println!();

    // print the seats
    let seats = environment.get_all_seats();
    println!("-> Compositor advertised {} seats:", seats.len());
    for seat in seats {
        smithay_client_toolkit::seat::with_seat_data(&seat, |data| {
            println!("  -> {} with capabilities: ", data.name);
            if data.has_pointer {
                print!("pointer ");
            }
            if data.has_keyboard {
                print!("keyboard ");
            }
            if data.has_touch {
                print!("touch ");
            }
            println!();
        });
    }
}

impl MainLoop {
    pub fn new() -> Self {
        let (environment, display, queue) = new_default_environment!(MyApp, desktop).unwrap();

        #[cfg(debug_assertions)]
        debug_printout(&environment);

        MainLoop {
            wl_display: display,
            main_event_queue: queue,
            windows: HashMap::new(),
            environment,
            callback: None,
        }
    }

    pub fn get_environment(&self) -> &Environment<MyApp> {
        &self.environment
    }
}

impl InnerLoop for MainLoop {
    fn wake_up(&self) {
        todo!()
    }

    fn init(&mut self, callback: impl FnMut(&Event)) {
        let callback = unsafe {
            mem::transmute::<Rc<RefCell<dyn FnMut(&Event)>>, Rc<RefCell<dyn FnMut(&Event)>>>(
                Rc::new(RefCell::new(callback)),
            )
        };
        self.callback = Some(callback);
        (self.callback.as_ref().unwrap().as_ref().borrow_mut())(&Event::Init);
    }

    fn process(&mut self, flow: &InnerFlow) {
        /*eprintln!(
            "Inside Main Loop. Wayland connection is alive: {}",
            self.wl_display.is_alive()
        );*/

        if let Some(err) = self.wl_display.protocol_error() {
            eprintln!(
                "Protocoll error:\nCode: {}\nMessage: {}\nObject Id: {}\nObject Interface: {}",
                err.code, err.message, err.object_id, err.object_interface
            );
        }
        self.wl_display
            .flush()
            .expect("Could not transfer data to the Wayland compositor\ndislplay.flush() failed!");

        //let mut to_delete;
        //Next action handling

        let mut followup_map = HashMap::new();

        for (id, mut window) in self.windows.drain() {
            {
                let mut shared = window.shared.as_ref().borrow_mut();
                match shared.take_next_action() {
                    NextAction::None => {}
                    NextAction::Refresh => {
                        window.window.refresh();
                        window.window.surface().commit();
                    }
                    NextAction::Redraw => {
                        shared.signal_drawen_once();
                        // Surface::redraw(&mut self.pool, window.window.surface(), shared.size.width, shared.size.height)
                        (self.callback.as_ref().unwrap().as_ref().borrow_mut())(
                            &Event::SurfaceEvent {
                                id,
                                event: crate::surface::event::Event::Redraw,
                            },
                        );
                        window.window.refresh();
                        window.window.surface().commit();
                    }
                    NextAction::Close => {
                        continue;
                    }
                    NextAction::Resize => {
                        window.window.resize(shared.size.width, shared.size.height);
                        window.window.refresh();
                        (self.callback.as_ref().unwrap().as_ref().borrow_mut())(
                            &Event::SurfaceEvent {
                                id,
                                event: crate::surface::event::Event::Resized(Extent {
                                    width: shared.size.width,
                                    height: shared.size.height,
                                }),
                            },
                        );
                        window.window.surface().commit();
                    }
                }
            }
            followup_map.insert(id, window);
        }
        self.windows = followup_map;
        //self.windows.remove(to_delete);

        match flow {
            InnerFlow::Wait => {
                //eprintln!("Inside wait");
                match self.main_event_queue.dispatch(&mut (), |raw_event, _, _| {
                    eprintln!("Got unhandled raw event: {:#?}", raw_event);
                }) {
                    Ok(_) => {}
                    Err(_) => {
                        eprintln!("Could not dispatch data!");
                        if let Some(err) = self.wl_display.protocol_error() {
                            eprintln!(
                                "Protocoll error:\nCode: {}\nMessage: {}\nObject Id: {}\nObject Interface: {}",
                                err.code, err.message, err.object_id, err.object_interface
                            );
                        }
                    }
                }
            }
            InnerFlow::Poll => {
                /*
                eprintln!("Inside poll");
                if let Err(e) = self.wl_display.flush() {
                    if e.kind() != ::std::io::ErrorKind::WouldBlock {
                        eprintln!("Error while trying to flush the wayland socket: {:?}", e);
                    }
                }*/

                //eprintln!("Inside poll inb4 prepare_read");
                //eprintln!("Inside poll inb4 prepare_read");
                if let Some(guard) = self.main_event_queue.prepare_read() {
                    match guard.read_events() {
                        Ok(_) => {
                            //eprintln!("Successfully read events from queue")
                        }
                        Err(e) => {
                            eprintln! {"Got error when reading events from queue: {}", e}
                        }
                    }
                    //eprintln!("Inside poll inb4 dispatch_pending");
                    self.main_event_queue
                        .dispatch_pending(&mut (), |raw_event, _, _| {
                            eprintln!("Got unhandled raw event: {:#?}", raw_event);
                        })
                        .expect("Failed to dispatch all messages.");
                }
            }
        }
    }
}
