use rui::instance::Instance;
use rui::surface::Surface;
use rui::util::Extent;

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
    //let root = component::component(Root);
    //instance.mount(&surface, root);

    instance.run()
}