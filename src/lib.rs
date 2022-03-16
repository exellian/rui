mod component;
mod util;
mod math;
mod renderer;
mod surface;
mod instance;
mod state;
mod event;

use url::Url;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);

        /*
        let instance = Instance::new();
        instance.create_window()
        let c = layer([

        ]);
        instance.create_window()
        instance.run(state).await;
        */
    }
}
