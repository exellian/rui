pub trait State {
    fn on_click<S>(&mut self, i: &mut S) {}
    fn on_hover<S>(&mut self, i: &mut S) {}
    fn on_key_press<S>(&mut self, i: &mut S) {}
    fn on_key_release<S>(&mut self, i: &mut S) {}
}