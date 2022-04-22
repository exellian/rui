use std::borrow::Cow;
use util::{Extent, point::Point};
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
    NotModal,
    ParentWindowModal,
    ApplicationModal,
}

#[derive(Debug, Clone)]
pub struct Attributes<'a> {
    pub title: Cow<'a, str>,
    pub position: Point,
    pub currentSize: Extent,
    pub minimumSize: Extent,
    pub maximumSize: Extent,
    pub windowState: WindowState,
    pub modality: Modality,
    pub isActive: bool,
    pub isResizable: bool,
    pub hasTitlebar: bool,
    pub hasMinimizeButton: bool,
    pub hasMaximizeButton: bool,
    pub hasCloseButton: bool,
    pub hasHelpButton: bool
}

impl Default for Attributes<'static> {
    fn default() -> Self {
        Attributes {
            title: Default::default(),
            position: Point { x: 0, y: 0 },
            currentSize: Extent { width: 480, height: 320 },
            minimumSize: Extent { width: 0, height: 0 },
            maximumSize: MAXIMUM_WINDOW_SIZE,
            windowState: WindowState::Hidden,
            modality: Modality::NotModal,
            isActive: true,
            isResizable: true,
            hasTitlebar: true,
            hasMinimizeButton: true,
            hasMaximizeButton: true,
            hasCloseButton: true,
            hasHelpButton: false
        }
    }
}

impl<'a> Attributes<'a> {

    pub fn new() -> Self {
        Attributes::default()
    }


}