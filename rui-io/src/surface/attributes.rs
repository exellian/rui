use std::borrow::Cow;
use rui_util::{Extent, point::Point};
use std::default::Default;

pub const MAXIMUM_WINDOW_SIZE: Extent = Extent {
    width: u32::MAX,
    height: u32::MAX,
};

#[derive(Debug, Copy, Clone)]
pub enum WindowState {
    Hidden = 0,
    Automatic = 1,
    Windowed = 2,
    Minimized = 3,
    Maximized = 4,
    Fullscreen = 5
}

#[derive(Debug, Copy, Clone)]
pub enum Modality {
    None,
    ParentWindow,
    Application,
}

#[derive(Debug, Clone)]
pub struct Attributes<'a> {
    pub title: Cow<'a, str>,
    pub position: Point,
    pub current_size: Extent,
    pub minimum_size: Extent,
    pub maximum_size: Extent,
    pub window_state: WindowState,
    pub modality: Modality,
    pub is_active: bool,
    pub is_resizable: bool,
    pub is_borderless: bool,
    pub has_minimize_button: bool,
    pub has_maximize_button: bool,
    pub has_close_button: bool,
    pub has_help_button: bool
}

impl Default for Attributes<'static> {
    fn default() -> Self {
        Attributes {
            title: Default::default(),
            position: Point { x: 0, y: 0 },
            current_size: Extent { width: 480, height: 320 },
            minimum_size: Extent { width: 0, height: 0 },
            maximum_size: MAXIMUM_WINDOW_SIZE,
            window_state: WindowState::Hidden,
            modality: Modality::None,
            is_active: true,
            is_resizable: true,
            is_borderless: false,
            has_minimize_button: true,
            has_maximize_button: true,
            has_close_button: true,
            has_help_button: false
        }
    }
}

impl<'a> Attributes<'a> {

    pub fn new() -> Self {
        Attributes::default()
    }


}
