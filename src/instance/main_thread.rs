use std::sync::mpsc;
use crate::Backend;
use crate::surface::{SurfaceAttributes, SurfaceFactory};

pub enum MainThreadRequest<B> where B: Backend {
    CreateSurface {
        attributes: SurfaceAttributes,
        sender: mpsc::Sender<Result<B::Surface, <B::SurfaceFactory as SurfaceFactory>::Error>>
    }
}

