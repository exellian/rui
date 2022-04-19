#![feature(downcast_unchecked)]

extern crate core;

mod event;
mod surface;
mod platform;
mod os_error;
mod io;
mod runtime;
pub mod alloc;



#[cfg(test)]
mod tests {


    unsafe fn as_mut<T>(x: &T) -> &mut T {
        &mut *(x as *const T as *mut T)
    }

}
