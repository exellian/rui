pub mod alloc;
pub mod extent;
pub mod ffi;
pub mod lazy;
pub mod offset;
pub mod point;

pub use extent::Extent;
pub use offset::Offset;
use std::time::SystemTime;

#[macro_export]
macro_rules! bs {
    ($name: ident) => {
        let $name = std::time::SystemTime::now();
    };
}

#[macro_export]
macro_rules! be {
    ($name: ident) => {
        println!(
            "{}: {}ms",
            stringify!($name),
            std::time::SystemTime::now()
                .duration_since($name)
                .unwrap()
                .as_millis()
        );
    };
}

pub fn bench<T>(name: &str, callback: impl FnOnce() -> T) -> T {
    let render = SystemTime::now();
    let res = callback();
    println!(
        "{}: {}",
        name,
        SystemTime::now()
            .duration_since(render)
            .unwrap()
            .as_millis()
    );
    res
}
