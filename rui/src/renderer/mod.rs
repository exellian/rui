mod renderer;
pub mod wgpu;

pub use renderer::Renderer;

#[derive(Copy, Clone)]
pub enum MSAA {
    X1,
    X4,
}
impl Into<u32> for MSAA {
    fn into(self) -> u32 {
        match self {
            MSAA::X1 => 1,
            MSAA::X4 => 4,
        }
    }
}
