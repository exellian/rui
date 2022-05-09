use rui_util::Extent;

#[derive(Clone, Debug)]
pub enum Event {
    Resized(Extent),
    Redraw,
    ShouldClose,
}
