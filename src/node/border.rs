use crate::math::{Vec4};
use crate::Node;

pub struct BorderNode {
    width: Vec4,
    radius: Vec4,
    inset: bool,
    node: Box<Node>
}