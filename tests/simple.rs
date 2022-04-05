use rui::component;
use rui::component::Component;
use rui::instance::{Backend, Instance};
use rui::surface::Surface;
use rui::util::Extent;
use async_trait::async_trait;
use rui::component::context::Context;
use rui::node::{Node, rect, component, image, path, bezier};

struct Root;

#[async_trait]
impl Component for Root {

    async fn on_event<B>(&mut self, ctx: &mut Context<B>) where Self: Sized, B: Backend {
        todo!()
    }

    async fn node(&mut self) -> Node {
        path([22, 234, 0], [
            bezier([0.0, 0.0], [1.0, 1.0], [0.3, 0.8], [0.8, 0.2])
        ])
        //image("test.jpeg", [0.05, 0.05, 0.05,  0.1])
        //rect([22, 234, 0], [0.05, 0.05, 0.05,  0.1])
    }
}

#[tokio::main]
async fn main() {
    let mut instance = Instance::default();

    let surface = Surface::builder()
        .title("Test")
        .size(Extent {
            width: 720,
            height: 720
        })
        .build(&instance)
        .expect("Failed to create window!");
    let root = component(Root);
    instance.mount(&surface, root).expect("Failed to mount root component!");
    instance.run()
}