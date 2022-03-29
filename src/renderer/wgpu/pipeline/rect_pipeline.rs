use std::borrow::Cow;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu_types::BufferUsages;
use crate::renderer::wgpu::primitive;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Instance {
    pub(crate) rect: [f32;4],
    pub(crate) color: [f32;4]
}
impl Instance {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct RectPipeline {
    pipeline: wgpu::RenderPipeline,
    instance_count: usize,
    instance_buffer: Option<wgpu::Buffer>
}
impl RectPipeline {

    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../../../../shader/rect.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Instance::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[config.format.into()],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        RectPipeline {
            pipeline,
            instance_count: 0,
            instance_buffer: None
        }
    }

    pub fn record<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.instance_count > 0 {
            // If instance_count > 0 then the instance buffer must exist
            debug_assert!(self.instance_buffer.is_some());
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.instance_buffer.as_ref().unwrap().slice(..));
            render_pass.draw(0..6, 0..self.instance_count as _);
        }
    }

    pub(crate) fn mount(&mut self, device: &wgpu::Device, rects: &Vec<primitive::Rect>) {
        if rects.len() == 0 {
            return;
        }
        // Use write buffer instead of recreation in the future.
        //      queue.write_buffer(&self.instance_buffer, offset, updated_data)
        // But this requires memory management.
        // For minimal working version recreate the buffer every frame
        self.instance_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(rects),
            usage: BufferUsages::VERTEX
        }));
        self.instance_count = rects.len();
    }
}