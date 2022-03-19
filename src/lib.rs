mod node;
mod util;
mod math;
mod renderer;
mod surface;
mod instance;
mod event;
mod component;
use crate::component::Component;
use crate::instance::Backend;
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

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::instance::Instance;
    use crate::surface::Surface;
    use crate::util::Extent;

    #[test]
    fn it_works() {
        let mut instance = Instance::default();
        let surface = Surface::builder()
            .title("Test")
            .size(Extent {
                width: 1280,
                height: 720
            })
            .build(&instance)
            .expect("Failed to create window!");
        
        let root = component::component(Root);
        instance.mount(&surface, root);
        instance.run()
    }
}
