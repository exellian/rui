use util::Extent;
use crate::os_error::OsError;
use crate::runtime::Runtime;
use crate::surface::Surface;
use std::borrow::Cow;
use crate::surface::attributes::{Attributes, WindowState, Modality};
use util::point::Point;

pub struct Builder<'a> {
    attributes: Attributes<'a>
}
impl<'a> Builder<'a> {

    pub fn new() -> Self {
        Builder {
            attributes: Attributes::default(),
        }
    }

    pub fn with_title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.attributes.title = title.into();
        self
    }

    pub fn with_position(mut self, position: Point) -> Self {
        self.attributes.position = position;
        self
    }

    pub fn with_size(mut self, size: Extent) -> Self {
        self.attributes.current_size = size;
        self
    }

    pub fn with_minimum_size(mut self, size: Extent) -> Self {
        self.attributes.minimum_size = size;
        self
    }

    pub fn with_maximum_size(mut self, size: Extent) -> Self {
        self.attributes.maximum_size = size;
        self
    }

    pub fn with_window_state(mut self, state: WindowState) -> Self {
        self.attributes.window_state = state;
        self
    }

    pub fn with_modality(mut self, modality: Modality) -> Self {
        self.attributes.modality = modality;
        self
    }

    pub fn with_active_flag(mut self, active: bool) -> Self {
        self.attributes.is_active = active;
        self
    }

    pub fn with_resizability_flag(mut self, resizable: bool) -> Self {
        self.attributes.is_resizable = resizable;
        self
    }

    pub fn with_titlebar(mut self, titlebar_enabled: bool) -> Self {
        self.attributes.has_titlebar = titlebar_enabled;
        self
    }

    pub fn with_minimize_button(mut self, minimize_button_enabled: bool) -> Self {
        self.attributes.has_minimize_button = minimize_button_enabled;
        self
    }

    pub fn with_maximize_button(mut self, maximize_button_enabled: bool) -> Self {
        self.attributes.has_maximize_button = maximize_button_enabled;
        self
    }

    pub fn with_close_button(mut self, close_button_enabled: bool) -> Self {
        self.attributes.has_close_button = close_button_enabled;
        self
    }

    pub fn with_help_button(mut self, help_button_enabled: bool) -> Self {
        self.attributes.has_help_button = help_button_enabled;
        self
    }
    
    pub fn build(self, runtime: &Runtime) -> Result<Surface, OsError> {
        Surface::try_from(&self.attributes)
    }
}