//! rui-io offers the low-level building blocks of a rui application.
//!
//! Provides abstraction over different operating system specific functions like creating
//! surfaces and retrieving events from the OS.
//!
//! Currently the following operating systems are supported:
//!
//!  - macOS
//!  - Windows
//!  - Linux with a Wayland compositor

#![feature(thread_spawn_unchecked)]

#[macro_use]
#[cfg(any(target_os = "ios", target_os = "macos"))]
extern crate objc;

#[cfg(target_os = "linux")]
extern crate smithay_client_toolkit;

#[macro_use]
extern crate lazy_static;

/// This module contains the low level implementation of an event loop aswell as a definition
/// of events.
pub mod event;
mod os_error;
mod platform;
/// This module offers the low level implementation of a surface with a drawable area.
pub mod surface;

pub use os_error::OsError;

#[cfg(test)]
mod tests {
    unsafe fn as_mut<T>(x: &T) -> &mut T {
        &mut *(x as *const T as *mut T)
    }
}
