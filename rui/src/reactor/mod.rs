use crate::instance::{Instance, InstanceShared};
use crate::Backend;
use rui_util::lazy::Lazy;
use std::future::Future;

static INSTANCE: Lazy<Reactor> = Lazy::new();

/// A reactor is a singleton that stores necessary
/// information about the runtime
pub struct Reactor {
    pub(crate) shared: InstanceShared,
}
impl Reactor {
    pub(crate) fn get() -> &'static Self {
        match INSTANCE.get() {
            Some(r) => r,
            None => panic!("Rui reactor not running!"),
        }
    }

    pub fn run<B>(
        instance: Instance<B>,
        shared: InstanceShared,
        start_app: impl Future<Output = ()>,
    ) -> !
    where
        B: Backend,
    {
        INSTANCE.init(Reactor::new(shared));
        instance.run(start_app)
    }

    fn new(shared: InstanceShared) -> Self {
        Reactor { shared }
    }
}
