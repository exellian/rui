extern crate alloc;
extern crate core;

pub use rui_macros::main;

use crate::component::Component;
use crate::instance::backend::Backend;
use crate::node::Node;

pub mod component;
pub mod error;
pub mod instance;
pub mod math;
pub mod node;
pub mod reactor;
pub mod renderer;
pub mod surface;
pub mod util;

/*
impl State for Mybutton {

    fn on_click(&mut self) {

    }

    fn on_event(&mut self) {

    }
}
*/

pub enum UserEvent {}
