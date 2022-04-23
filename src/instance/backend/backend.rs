use crate::renderer::Renderer;

pub trait Backend: Sync + Send + Sized {
    
    /// The renderer
    type Renderer: Renderer<Self> + Send;

}