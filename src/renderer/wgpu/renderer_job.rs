pub struct RenderJob {
    surface: wgpu::Surface,
    pipeline: wgpu::RenderPipeline,
    command_buffer: wgpu::CommandBuffer
}