use crate::surface::Surface;
use rui_io::surface::{Modality, SurfaceAttributes, WindowState};
use rui_util::point::Point;
use rui_util::Extent;
use std::borrow::Cow;

pub struct Builder<'a> {
    attributes: SurfaceAttributes<'a>,
}
impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Builder {
            attributes: SurfaceAttributes::default(),
        }
    }

    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.attributes.title = title.into();
        self
    }

    pub fn position(mut self, position: Point) -> Self {
        self.attributes.position = position;
        self
    }

    pub fn size(mut self, size: Extent) -> Self {
        self.attributes.current_size = size;
        self
    }

    pub fn minimum_size(mut self, size: Extent) -> Self {
        self.attributes.minimum_size = size;
        self
    }

    pub fn maximum_size(mut self, size: Extent) -> Self {
        self.attributes.maximum_size = size;
        self
    }

    pub fn window_state(mut self, state: WindowState) -> Self {
        self.attributes.window_state = state;
        self
    }

    pub fn modality(mut self, modality: Modality) -> Self {
        self.attributes.modality = modality;
        self
    }

    pub fn active_flag(mut self, active: bool) -> Self {
        self.attributes.is_active = active;
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.attributes.is_resizable = resizable;
        self
    }

    pub fn borderless(mut self, borderless: bool) -> Self {
        self.attributes.is_borderless = borderless;
        self
    }

    pub fn minimize_button(mut self, minimize_button_enabled: bool) -> Self {
        self.attributes.has_minimize_button = minimize_button_enabled;
        self
    }

    pub fn maximize_button(mut self, maximize_button_enabled: bool) -> Self {
        self.attributes.has_maximize_button = maximize_button_enabled;
        self
    }

    pub fn close_button(mut self, close_button_enabled: bool) -> Self {
        self.attributes.has_close_button = close_button_enabled;
        self
    }

    pub fn help_button(mut self, help_button_enabled: bool) -> Self {
        self.attributes.has_help_button = help_button_enabled;
        self
    }

    pub async fn build(self) -> Surface {
        todo!()
    }
}
