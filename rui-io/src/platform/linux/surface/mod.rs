use crate::surface::SurfaceAttributes;
use crate::event::LoopTarget;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub struct Surface<'main, 'child> {
    main: &'main str,
    child: &'child str,
}

impl<'main, 'child> Surface<'main, 'child> {
    pub fn new(loop_target: &LoopTarget<'main, 'child>, attr: &SurfaceAttributes) -> Self {
        todo!()
    }
}
unsafe impl<'main, 'child> HasRawWindowHandle for Surface<'main, 'child> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        todo!()
    }
}