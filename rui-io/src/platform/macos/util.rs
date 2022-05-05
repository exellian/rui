use crate::event::{Event, LoopTarget};
use crate::platform;
use crate::surface::SurfaceId;
use cocoa::base::id;
use objc::runtime::{Class, Object};
use std::pin::Pin;

pub fn get_window_id(window_cocoa_id: id) -> SurfaceId {
    SurfaceId::from(window_cocoa_id as *const Object as u64)
}

pub unsafe fn superclass(this: &Object) -> &Class {
    let superclass: *const Class = msg_send![this, superclass];
    &*superclass
}
