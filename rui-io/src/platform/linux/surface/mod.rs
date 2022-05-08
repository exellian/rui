use crate::event::LoopTarget;
use crate::surface::{SurfaceAttributes, SurfaceId};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use rui_util::Extent;
use smithay_client_toolkit::shm::{AutoMemPool, Format};
use smithay_client_toolkit::shm::{DoubleMemPool, MemPool};
use smithay_client_toolkit::window::{Event, FallbackFrame, Window};
use std::io::{BufWriter, Write};
use wayland_client::protocol::wl_surface::WlSurface;

enum NextAction {
    Refresh,
    Redraw,
    Exit,
}

pub struct Surface<'main, 'child> {
    loop_target: LoopTarget<'main, 'child>,
    window: Window<FallbackFrame>,
    close_requested: bool,
    refresh_requested: bool,
    size: Extent,
    next_action: Option<NextAction>,
    has_drawn_once: bool,
    pool: AutoMemPool,
}

impl<'main, 'child> Surface<'main, 'child> {
    pub fn new(
        loop_target: &LoopTarget<'main, 'child>,
        attr: &SurfaceAttributes,
    ) -> Surface<'main, 'child> {
        let main_loop = match loop_target {
            LoopTarget::Main(ml) => *ml,
            LoopTarget::Child(child) => {
                eprintln!("Inside Main LoopTarget");
                child.main
            }
        };

        let mut size_x = attr.current_size.width;
        let mut size_y = attr.current_size.height;

        let inner_ml = main_loop.inner.borrow();
        let environment = inner_ml.get_environment();
        let surface = environment
            .create_surface_with_scale_callback(|dpi, _surface, _dispatch_data| {
                println!("dpi changed to {}", dpi);
            })
            .detach();

        let mut pool = environment
            .create_auto_pool()
            .expect("Could not create buffer pool");

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

        let mut window = environment
            .create_window::<FallbackFrame, _>(
                surface,
                None,
                (size_x, size_y),
                |event, mut dispatch_data| {
                    eprintln!("Got event: {:#?}", event);
                    let state = dispatch_data.get::<Surface>().unwrap();
                    match event {
                        Event::Configure { new_size, states } => {
                            if let Some(size) = new_size {
                                state.size.height = size.0;
                                state.size.width = size.1;
                                state.refresh_requested = true;
                            }
                        }
                        Event::Close => {
                            state.close_requested = true;
                        }
                        Event::Refresh => {
                            state.refresh_requested = true;
                            state.window.refresh();
                        }
                    }
                },
            )
            .expect("Unable to create new window");
        window.surface().attach(Some(&buffer.1), 0, 0);
        window
            .surface()
            .damage_buffer(0, 0, size_x as i32, size_y as i32);
        window.surface().commit();

        if !attr.title.is_empty() {
            window.set_title(attr.title.clone());
        }

        // window.refresh();

        let mut win = Surface {
            loop_target: loop_target.clone(),
            window,
            close_requested: false,
            refresh_requested: false,
            size: Extent {
                width: size_x,
                height: size_y,
            },
            next_action: None,

            has_drawn_once: false,
            pool: pool,
        };

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
        todo!()
    }
}
