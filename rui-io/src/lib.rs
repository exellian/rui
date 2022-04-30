#[macro_use]
extern crate objc;

#[macro_use]
extern crate lazy_static;

pub mod event;
pub mod surface;
mod platform;
mod os_error;

pub use os_error::OsError;

#[cfg(test)]
mod tests {
    unsafe fn as_mut<T>(x: &T) -> &mut T {
        &mut *(x as *const T as *mut T)
    }
}
