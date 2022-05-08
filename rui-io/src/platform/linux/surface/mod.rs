use std::cell::RefCell;
use crate::event::LoopTarget;
use crate::surface::{SurfaceAttributes, SurfaceId};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle, WaylandHandle};
use rui_util::Extent;
use smithay_client_toolkit::shm::{AutoMemPool, Format};
use smithay_client_toolkit::shm::{DoubleMemPool, MemPool};
use smithay_client_toolkit::window::{Event, FallbackFrame, Window};
use std::io::{BufWriter, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use wayland_client::Display;
use wayland_client::protocol::wl_surface::WlSurface;
use rui_util::alloc::oneshot;
use crate::platform::event::{WindowState, WindowStateShared};

enum NextAction {
    Refresh,
    Redraw,
    Exit,
}

pub struct Surface<'main, 'child> {
    loop_target: LoopTarget<'main, 'child>,
    wl_display: Display,
    wl_surface: WlSurface,
    surface_id: SurfaceId,
    size: Extent
}

impl<'main, 'child> Surface<'main, 'child> {
    pub async fn new(
        loop_target: &LoopTarget<'main, 'child>,
        attr: &SurfaceAttributes,
    ) -> Surface<'main, 'child> {
        eprintln!("Inside new surface!");
        let main_loop = match loop_target {
            LoopTarget::Main(ml) => *ml,
            LoopTarget::Child(_) => {
                panic!("Not supporting surface creation on non main thread for now!");
            }
        };

        let mut size_x = attr.current_size.width;
        let mut size_y = attr.current_size.height;

        let (sender, mut receiver) = oneshot::channel::<()>();
        let at_least_configured_once = Arc::new(AtomicBool::new(false));
        let win = {
            let mut inner_ml = main_loop.inner.borrow_mut();
            let environment = inner_ml.get_environment();
            if !environment.get_shell().unwrap().needs_configure() {
                eprintln!("Shell needs configure from us!");
            }
            let surface = environment
                .create_surface_with_scale_callback(|dpi, _surface, _dispatch_data| {
                    // todo
                    println!("dpi changed to {}", dpi);
                })
                .detach();


            let surface_id = Self::surface_id(&surface);

            let window_state_shared = Arc::new(RefCell::new(WindowStateShared::new(Extent {
                width: size_x,
                height: size_y
            })));

            let sender_arc = Arc::new(RefCell::new(Some(sender)));
            let surface_cloned = surface.clone();
            let window_state_shared_cloned = window_state_shared.clone();

            let window = environment
                .create_window::<FallbackFrame, _>(
                    surface.clone(),
                    None,
                    (size_x, size_y),
                    move |event, _| {
                        eprintln!("Got event: {:#?}", event);
                        //let state = dispatch_data.get::<Surface>().unwrap();

                        let surface = surface_cloned.clone();
                        let window_state_shared = window_state_shared_cloned.clone();
                        let mut window_state_shared_mut = window_state_shared.as_ref().borrow_mut();

                        match event {
                            Event::Configure { new_size, states } => {

                                if let Some(new_size) = new_size {
                                    window_state_shared_mut.set_size(Extent {
                                        width: new_size.0,
                                        height: new_size.1
                                    });
                                }

                                if !window_state_shared_mut.is_drawen_once() {
                                    window_state_shared_mut.signal_should_redraw();
                                    let sender = sender_arc.as_ref().borrow_mut().take().unwrap();
                                    sender.send(());
                                } else {
                                    window_state_shared_mut.signal_should_refresh();
                                }
                            }
                            Event::Close => {
                                //state.close_requested = true;
                            }
                            Event::Refresh => {
                                //state.refresh_requested = true;
                                //state.window.refresh();
                                window_state_shared_mut.signal_should_refresh();
                            }
                        }
                    },
                )
                .expect("Unable to create new window");

            if !attr.title.is_empty() {
                window.set_title(attr.title.clone());
            }
            inner_ml.windows.insert(surface_id, WindowState::new(window, window_state_shared)); // :/

            let win = Surface {
                loop_target: loop_target.clone(),
                wl_display: inner_ml.wl_display.clone(),
                wl_surface: surface,
                surface_id,
                size: Extent {
                    width: size_x,
                    height: size_y,
                }
            };
            win
        };
        receiver.recv().await;
        win
    }

    pub fn redraw(pool: &mut AutoMemPool, surface: &WlSurface, size_x: u32, size_y: u32) {
        let mut buffer = pool
            .buffer(
                size_x as i32,
                size_y as i32,
                (size_x * 4) as i32,
                Format::Argb8888,
            )
            .expect("Could not create buffer");

        {
            let pxcount = size_x * size_y;
            let mut writer = BufWriter::new(&mut buffer.0);
            let pixel: u32 = 0xFF_7C_7C_7C;
            for _ in 0..pxcount {
                writer.write_all(&pixel.to_ne_bytes()).unwrap();
            }
            writer.flush().unwrap();
        }
        surface.attach(Some(&buffer.1), 0, 0);
        surface.damage_buffer(0, 0, size_x as i32, size_y as i32);
        surface.commit();
    }

    fn surface_id(surface: &WlSurface) -> SurfaceId {
        SurfaceId::from(surface.as_ref().id() as u64)
    }

    pub fn inner_size(&self) -> Extent {
        todo!()
    }

    pub fn id(&self) -> SurfaceId {
        self.surface_id
    }

    pub fn request_redraw(&mut self) {
        //self.refresh_requested = true;
    }
}
unsafe impl<'main, 'child> HasRawWindowHandle for Surface<'main, 'child> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = WaylandHandle::empty();
        handle.display = self.wl_display.get_display_ptr() as *const _ as *mut _;
        handle.surface = self.wl_surface.as_ref().c_ptr() as *const _ as *mut _;
        RawWindowHandle::Wayland(handle)
    }
}
