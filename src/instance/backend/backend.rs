use crate::renderer::Renderer;

pub trait Backend: Sync + Send + Sized {
    
    /// The renderer to use that supports the given surface
    type Renderer: Renderer<Self> + Send;

}