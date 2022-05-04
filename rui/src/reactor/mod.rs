use rui_util::lazy::Lazy;

static INSTANCE: Lazy<Reactor> = Lazy::new();

/// A reactor is a singleton that stores necessary
/// information about the runtime
pub struct Reactor {}
impl Reactor {
    pub fn init() {
        INSTANCE.init(Reactor::new())
    }

    pub(crate) fn get() -> &'static Self {
        match INSTANCE.get() {
            Some(r) => r,
            None => panic!("Rui reactor not running!"),
        }
    }

    fn new() -> Self {
        Reactor {}
    }
}
