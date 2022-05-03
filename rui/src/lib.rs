extern crate alloc;
extern crate core;

pub mod component;
pub mod instance;
pub mod math;
pub mod node;
pub mod renderer;
pub mod surface;
pub mod util;

use crate::component::context::Context;
use crate::component::Component;
use crate::instance::Backend;
use crate::node::Node;
use async_trait::async_trait;

/*
impl State for Mybutton {

    fn on_click(&mut self) {

    }

    fn on_event(&mut self) {

    }
}
*/

pub enum UserEvent {}
