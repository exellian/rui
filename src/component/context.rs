use crate::instance::{Backend, Instance};
use crate::node::base::BaseNode;

pub struct Context<B> where B: Backend {
    base: BaseNode,
    instance: Instance<B>,
}