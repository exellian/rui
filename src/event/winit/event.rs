use winit::event::WindowEvent;
use crate::event::event::Event;
use crate::surface::SurfaceEvent;
use crate::util::Extent;

impl<'a, T> TryFrom<winit::event::Event<'a, T>> for Event<T> {
    type Error = ();

    fn try_from(value: winit::event::Event<'a, T>) -> Result<Self, Self::Error> {
        match value {
            winit::event::Event::UserEvent(v) => Ok(Event::UserEvent(v)),
            winit::event::Event::RedrawRequested(window_id) => {
                Ok(Event::SurfaceEvent {
                    id: window_id.into(),
                    event: SurfaceEvent::Redraw
                })
            },
            winit::event::Event::WindowEvent { event, window_id } => match event {
                WindowEvent::Resized(size) => {
                    Ok(Event::SurfaceEvent {
                        id: window_id.into(),
                        event: SurfaceEvent::Resized(Extent {
                            width: size.width,
                            height: size.height
                        })
                    })
                }
                _ => Err(())
            }
            _ => Err(())
        }
    }
}