use tokio::sync::mpsc;
use crate::event::{Event, EventLoop};
use crate::instance::backend::Backend;
use crate::Node;
use crate::util::Sender;

pub struct Instance<B> where
    B: Backend
{
    event_loop: B::EventLoop,
    renderer: B::Renderer
}
impl<B> Instance<B> where
    B: Backend
{

    pub fn new(event_loop: B::EventLoop, renderer: B::Renderer) -> Self {
        Instance {
            event_loop,
            renderer
        }
    }

    pub fn run(self, root: &Node) {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.event_loop.run(Sender::new(sender));

    }
}