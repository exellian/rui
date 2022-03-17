mod node;
mod util;
mod math;
mod renderer;
mod surface;
mod instance;
mod event;
mod component;
use crate::component::Component;
use crate::component::context::Context;
use crate::instance::Backend;
use crate::node::Node;

struct Mybutton {
    pressed: bool,
}

impl Component for Mybutton {

    fn node<B>(&mut self, _: &mut Context<B>) -> Node where Self: Sized, B: Backend {
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        /*
                let r = Renderer::default();
                let l = EventLoop::default();
                Instance::new(&r, &l).await;

                let instance = Instance::new();
                instance.create_window()
                let c = layer([

                ]);
                instance.create_window()
                instance.run(state).await;
                */
    }
}
