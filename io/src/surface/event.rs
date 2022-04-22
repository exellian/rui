use util::Extent;

#[derive(Clone)]
pub enum Event {
    Resized(Extent),
    Redraw
}