use async_trait::async_trait;

use rui::component::Component;
use rui::instance::backend::Backend;
use rui::node::{comp, component, path, Node};
use rui::surface::Surface;
use rui_util::Extent;

struct Root;

#[async_trait]
impl Component for Root {
    async fn on_event<B>(&mut self)
    where
        Self: Sized,
        B: Backend,
    {
        todo!()
    }

    async fn node(&mut self) -> Node {
        let mut paths = vec![];

        for _ in 0..200 / 8 {
            paths.push(
                path([22, 234, 0], [0.1, 0.1])
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
                    .close(),
            );
        }

        comp(paths)
        //rect([22, 234, 0], [0.05, 0.05, 0.05,  0.1])
    }
}

#[rui::main]
async fn main() {
    let surface = Surface::builder()
        .resizable(true)
        .title("Test")
        .size(Extent {
            width: 1440,
            height: 900,
        })
        .build()
        .await;
    surface.mount(component(Root)).await;
}
