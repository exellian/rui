#![feature(thread_spawn_unchecked)]

#[macro_use]
#[cfg(any(target_os = "ios", target_os = "macos"))]
extern crate objc;

#[macro_use]
extern crate lazy_static;

pub mod event;
mod os_error;
mod platform;
pub mod surface;

pub use os_error::OsError;

#[cfg(test)]
mod tests {
    unsafe fn as_mut<T>(x: &T) -> &mut T {
        &mut *(x as *const T as *mut T)
    }
}
