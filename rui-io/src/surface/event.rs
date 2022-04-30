use rui_util::Extent;

#[derive(Clone)]
pub enum Event {
    Resized(Extent),
    Redraw,
    ShouldClose,
}