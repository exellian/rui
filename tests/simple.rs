use rui::component;
use rui::component::Component;
use rui::instance::{Backend, Instance};
use rui::surface::Surface;
use rui::util::Extent;
use async_trait::async_trait;
use rui::component::context::Context;
use rui::node::{Node, rect, component};

struct Root;

#[async_trait]
impl Component for Root {

    async fn on_event<B>(&mut self, ctx: &mut Context<B>) where Self: Sized, B: Backend {
        todo!()
    }

    async fn node(&self) -> Node {
        rect([234, 22, 0])
    }
}

fn main() {
    let mut instance = Instance::default();

    let surface = Surface::builder()
        .title("Test")
        .size(Extent {
            width: 1280,
            height: 720
        })
        .build(&instance)
        .expect("Failed to create window!");
    let root = component(Root);
    instance.mount(&surface, root).expect("Failed to mount root component!");

    instance.run()
}