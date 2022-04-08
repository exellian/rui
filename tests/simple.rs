use rui::component::Component;
use rui::instance::{Backend, Instance};
use rui::surface::Surface;
use rui::util::Extent;
use async_trait::async_trait;
use rui::component::context::Context;
use rui::node::{Node, component, path, comp};

struct Root;

#[async_trait]
impl Component for Root {

    async fn on_event<B>(&mut self, ctx: &mut Context<B>) where Self: Sized, B: Backend {
        todo!()
    }

    async fn node(&mut self) -> Node {
        let mut paths = vec![];

        for _ in 0..200/8 {
            paths.push(path([22, 234, 0], [0.1, 0.1])
                .cubic_bezier([0.9, 0.1], [0.2, 0.05], [0.8, 0.15])
                //.linear([0.9, 0.1])
                .linear([0.9, 0.3])
                .linear([0.6, 0.3])
                .cubic_bezier([0.7, 0.6], [0.8, 0.4], [0.5, 0.5])
                //.linear([0.6, 0.6])
                .linear([0.8, 0.6])
                .linear([0.8, 0.8])
                .linear([0.4, 0.8])
                .cubic_bezier([0.1, 0.2], [0.1, 0.7], [0.7, 0.3])
                .close());
        }

        comp(paths)
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