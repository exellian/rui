extern crate core;

pub mod event;
pub mod surface;
mod platform;
mod os_error;
pub mod runtime;
pub mod alloc;

pub use os_error::OsError;

#[cfg(test)]
mod tests {
    unsafe fn as_mut<T>(x: &T) -> &mut T {
        &mut *(x as *const T as *mut T)
    }
}
