use crate::renderer::Renderer;

pub trait Backend: 'static + Sync + Send + Sized {
    /// The renderer
    type Renderer: Renderer<Self> + Send;
}
