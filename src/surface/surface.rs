use std::error::Error;

pub trait Surface {
    type EventLoop;
    type Error: Error;
    
}