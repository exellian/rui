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
use wayland_client::Display;
use wayland_client::protocol::wl_surface::WlSurface;
use rui_util::alloc::oneshot;

enum NextAction {
    Refresh,
    Redraw,
    Exit,
}

pub struct Surface<'main, 'child> {
    loop_target: LoopTarget<'main, 'child>,
    wl_display: Display,
    wl_surface: WlSurface,
    window: Window<FallbackFrame>,
    close_requested: bool,
    refresh_requested: bool,
    size: Extent,
    next_action: Option<NextAction>,
    has_drawn_once: bool,
    pool: AutoMemPool,
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
                panic!("Not supporting surface creation on non main thread for know!");
            }
        };

        let mut size_x = attr.current_size.width;
        let mut size_y = attr.current_size.height;

        let (sender, mut receiver) = oneshot::channel::<()>();

        let win = {
            let inner_ml = main_loop.inner.borrow();
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

            let mut pool = environment
                .create_auto_pool()
                .expect("Could not create buffer pool");



            let sender_arc = Arc::new(RefCell::new(Some(sender)));
            let env = environment.clone();
            let surface_cloned = surface.clone();
            let window = environment
                .create_window::<FallbackFrame, _>(
                    surface.clone(),
                    None,
                    (size_x, size_y),
                    move |event, mut _dispatch_data| {
                        eprintln!("Got event: {:#?}", event);
                        //let state = dispatch_data.get::<Surface>().unwrap();

                        match event {
                            Event::Configure { new_size, states } => {
                                let mut sender = sender_arc.clone();
                                let surface = surface_cloned.clone();
                                let mut pool = env
                                    .create_auto_pool()
                                    .expect("Could not create buffer pool");
                                if let Some(new_size) = new_size {
                                    size_x = new_size.0;
                                    size_y = new_size.1;
                                }
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
                                    let mut writer = BufWriter::new(buffer.0);
                                    let pixel: u32 = 0xFF_23_23_16;
                                    for _ in 0..pxcount {
                                        writer.write_all(&pixel.to_ne_bytes()).unwrap();
                                    }
                                    writer.flush().unwrap();
                                }
                                surface.attach(Some(&buffer.1), 0, 0);
                                surface
                                    .damage_buffer(0, 0, size_x as i32, size_y as i32);
                                surface.commit();
                                { println!("Surface content: {:#?}", surface); }
                                let sender = sender.as_ref().borrow_mut().take().unwrap();
                                sender.send(());
                            }
                            Event::Close => {
                                //state.close_requested = true;
                            }
                            Event::Refresh => {
                                //state.refresh_requested = true;
                                //state.window.refresh();
                            }
                        }
                    },
                )
                .expect("Unable to create new window");

            if !attr.title.is_empty() {
                window.set_title(attr.title.clone());
            }
            let win = Surface {
                loop_target: loop_target.clone(),
                wl_display: inner_ml.wl_display.clone(),
                wl_surface: surface,
                window,
                close_requested: false,
                refresh_requested: false,
                size: Extent {
                    width: size_x,
                    height: size_y,
                },
                next_action: None,

                has_drawn_once: false,
                pool,
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
                (size_y * 4) as i32,
                Format::Argb8888,
            )
            .expect("Could not create buffer");

        {
            let pxcount = size_x * size_y;
            let mut writer = BufWriter::new(&mut buffer.0);
            let pixel: u32 = 0xFF_D0_00_00;
            for _ in 0..pxcount {
                writer.write_all(&pixel.to_ne_bytes()).unwrap();
            }
            writer.flush().unwrap();
        }
        surface.attach(Some(&buffer.1), 0, 0);
        surface.damage_buffer(0, 0, size_x as i32, size_y as i32);
        surface.commit();
    }

    pub fn inner_size(&self) -> Extent {
        self.size.clone()
    }

    pub fn id(&self) -> SurfaceId {
        // todo
        SurfaceId::from(23)
    }

    pub fn request_redraw(&mut self) {
        self.refresh_requested = true;
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
