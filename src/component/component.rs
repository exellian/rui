use crate::component::context::Context;
use crate::instance::Backend;
use crate::node::Node;

/// A component represents a collection of nodes
/// and should hold the state of the component
/// and should handle incoming events on the component
pub trait Component {

    fn on_click<B>(&mut self, context: &mut Context<B>)
        where Self: Sized, B: Backend {}
    fn on_mouse_in<B>(&mut self, context: &mut Context<B>)
        where Self: Sized, B: Backend {}
    fn on_mouse_out<B>(&mut self, context: &mut Context<B>)
        where Self: Sized, B: Backend {}
    fn on_hover<B>(&mut self, context: &mut Context<B>)
        where Self: Sized, B: Backend {}

    fn node<B>(&mut self, context: &mut Context<B>) -> Node
        where Self: Sized, B: Backend;
}