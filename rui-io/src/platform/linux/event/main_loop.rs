use crate::event::queue::Dequeue;
use crate::event::{Event, Flow, InnerLoop};
use smithay_client_toolkit::environment::Environment;
use smithay_client_toolkit::reexports::client::{Attached, DispatchData, Display};
use smithay_client_toolkit::shell::Shell;
use smithay_client_toolkit::window::{Event as WEvent, FallbackFrame, State, Window};
use smithay_client_toolkit::{default_environment, new_default_environment};
use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::mem;
use std::rc::Rc;
use wayland_client::{EventQueue, ReadEventsGuard};

default_environment!(MyApp, desktop);

pub struct MainLoop {
    wl_display: Display,
    main_event_queue: EventQueue,
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
        let (environment, display, mut queue) = new_default_environment!(MyApp, desktop).unwrap();

        #[cfg(debug_assertions)]
        debug_printout(&environment);

        MainLoop {
            wl_display: display,
            main_event_queue: queue,
            environment,
            callback: None,
        }
    }

    pub fn get_environment(&self) -> &Environment<MyApp> {
        &self.environment
    }
    pub fn get_queue(&self) -> &EventQueue {
        &self.main_event_queue
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
    }

    fn process(&mut self, flow: &Flow) {
        eprintln!(
            "Inside Main Loop. Wayland connection is alive: {}",
            self.wl_display.is_alive()
        );
        if let Some(err) = self.wl_display.protocol_error() {
            eprintln!(
                "Protocoll error:\nCode: {}\nMessage: {}\nObject Id: {}\nObject Interface: {}",
                err.code, err.message, err.object_id, err.object_interface
            );
        }
        self.wl_display.flush();
        match flow {
            Flow::Wait => {
                eprintln!("Inside wait");
                self.main_event_queue
                    .sync_roundtrip(&mut (), |raw_event, _, _| {
                        eprintln!("Got unhandled raw event: {:#?}", raw_event);
                    });
            }
            Flow::Poll => {
                eprintln!("Inside poll");
                if let Err(e) = self.wl_display.flush() {
                    if e.kind() != ::std::io::ErrorKind::WouldBlock {
                        eprintln!("Error while trying to flush the wayland socket: {:?}", e);
                    }
                }

                eprintln!("Inside poll inb4 prepare_read");
                if let Some(guard) = self.main_event_queue.prepare_read() {
                    match guard.read_events() {
                        Ok(_) => {
                            eprintln!("Successfully read events from queue")
                        }
                        Err(e) => {
                            eprintln! {"Got error when reading events from queue: {}", e}
                        }
                    }
                    eprintln!("Inside poll inb4 dispatch_pending");
                    self.main_event_queue
                        .dispatch_pending(&mut (), |raw_event, _, _| {
                            eprintln!("Got unhandled raw event: {:#?}", raw_event);
                        })
                        .expect("Failed to dispatch all messages.");
                }
            }
            Flow::Exit(_) => {
                return;
            }
        }
    }
}
