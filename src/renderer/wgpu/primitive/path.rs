#[repr(C, align(8))]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PathSegment {
    pub typ: u32,
    pub flags: u32,
    pub param0: [f32; 2],
    pub param1: [f32; 2],
    pub param2: [f32; 2],
    pub param3: [f32; 2]
}
#[repr(C, align(8))]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Paths {
    pub segments: [PathSegment; 256]
}
pub struct Path {
    pub rect: [f32;4],
    pub color: [f32;4],
    pub segments: Vec<PathSegment>
}
