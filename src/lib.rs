extern crate core;

pub mod node;
pub mod util;
pub mod math;
pub mod renderer;
pub mod surface;
pub mod instance;
pub mod event;
pub mod component;

use crate::component::Component;
use crate::instance::{Backend, Instance};
use crate::node::Node;
use async_trait::async_trait;
use crate::component::context::Context;

struct Root;

#[async_trait]
impl Component for Root {

    async fn node<B>(&mut self, context: &mut Context<B>) -> Node where Self: Sized, B: Backend {
        todo!()
    }
}

/*
impl State for Mybutton {

    fn on_click(&mut self) {

    }

    fn on_event(&mut self) {

    }
}
*/

pub enum UserEvent {
    
}