use crate::node::path::PathNode;
use crate::renderer::wgpu::pipeline::new_path_pipeline::vertex::{
    ColorVertex, FanVertex, SegmentVertex,
};
use crate::renderer::wgpu::primitive;
use crate::renderer::MSAA;
use crate::util;
use crate::util::PathSegment;
use alloc::borrow::Cow;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu_types::{
    BlendComponent, BlendFactor, BlendOperation, BlendState, BufferUsages, ColorTargetState,
    ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Face, FrontFace, IndexFormat,
    PolygonMode, PrimitiveTopology, StencilFaceState, StencilOperation, StencilState,
    TextureFormat,
};

mod vertex {
    use std::mem;

    pub type ColorVertex = FanVertex;

    /// Represents a Vertex for the fan pipeline
    #[repr(C)]
    #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct FanVertex {
        pub pos: [f32; 2],
    }
    impl FanVertex {
        pub const DESCRIPTION: wgpu::VertexBufferLayout<'_> = wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<FanVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        };
    }

    /// Represents a Vertex for the fan pipeline
    #[repr(C)]
    #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct SegmentVertex {
        pub pos: [f32; 2],
        pub segment_index: u32,
    }
    impl SegmentVertex {
        pub const DESCRIPTION: wgpu::VertexBufferLayout<'_> = wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<SegmentVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        };
    }
}

struct Mount {
    fan_vertex_buffer: wgpu::Buffer,
    fan_index_buffer: wgpu::Buffer,
    fan_index_count: u32,

    segment_vertex_buffer: wgpu::Buffer,
    segment_index_buffer: wgpu::Buffer,
    segment_index_count: u32,

    segments_buffer: wgpu::Buffer,
    segments_buffer_bind_group: wgpu::BindGroup,

    color_vertex_buffer: wgpu::Buffer,
    color_index_buffer: wgpu::Buffer,
    color_index_count: u32,
}

pub struct PathPipeline {
    /// Global buffer holds information about width and height of the screen
    globals_buffer: wgpu::Buffer,
    globals_bind_group: wgpu::BindGroup,
    globals_bind_group_layout: wgpu::BindGroupLayout,

    /// Pipeline for rendering triangle fans
    /// to the stencil buffer
    fans_pipeline: wgpu::RenderPipeline,

    /// Pipeline for rendering curve-segments
    /// to the stencil buffer
    segments_convex_pipeline: wgpu::RenderPipeline,
    segments_concave_pipeline: wgpu::RenderPipeline,
    segments_buffer_bind_group_layout: wgpu::BindGroupLayout,

    /// Final pipeline which uses the stencil buffer to draw
    /// vector graphics
    color_pipeline: wgpu::RenderPipeline,

    /// Mount of the path pipeline
    mount: Option<Mount>,
}
impl PathPipeline {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, msaa: &MSAA) -> Self {
        let (globals_buffer, globals_bind_group_layout, globals_bind_group) =
            Self::build_globals_buffer(device, config);
        let fans_pipeline =
            Self::build_fans_pipeline(device, config, msaa, &globals_bind_group_layout);
        let segments_buffer_bind_group_layout =
            Self::build_segments_buffer_bind_group_layout(device);
        let segments_concave_pipeline = Self::build_segments_concave_pipeline(
            device,
            config,
            msaa,
            &segments_buffer_bind_group_layout,
            &globals_bind_group_layout,
        );
        let segments_convex_pipeline = Self::build_segments_convex_pipeline(
            device,
            config,
            msaa,
            &segments_buffer_bind_group_layout,
            &globals_bind_group_layout,
        );
        let color_pipeline = Self::build_color_pipeline(device, config, msaa);

        PathPipeline {
            globals_buffer,
            globals_bind_group,
            globals_bind_group_layout,
            fans_pipeline,
            segments_convex_pipeline,
            segments_concave_pipeline,
            segments_buffer_bind_group_layout,
            color_pipeline,
            mount: None,
        }
    }

    fn build_globals_buffer(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> (wgpu::Buffer, wgpu::BindGroupLayout, wgpu::BindGroup) {
        let globals = primitive::Globals {
            width_height: util::pack(config.width as u16, config.height as u16),
            aspect_ratio: config.width as f32 / config.height as f32,
        };

        let globals_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("globals_buffer"),
            contents: bytemuck::cast_slice(&[globals]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let globals_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("globals_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let globals_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &globals_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: globals_buffer.as_entire_binding(),
            }],
            label: Some("globals_bind_group"),
        });
        (
            globals_buffer,
            globals_bind_group_layout,
            globals_bind_group,
        )
    }

    fn build_color_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        msaa: &MSAA,
    ) -> wgpu::RenderPipeline {
        let multisample = wgpu::MultisampleState {
            count: msaa.clone().into(),
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../../../../shader/path/color.wgsl"
            ))),
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
                buffers: &[ColorVertex::DESCRIPTION],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: None,
                    write_mask: ColorWrites::default(),
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: false,
                depth_compare: CompareFunction::Always,
                stencil: StencilState {
                    front: StencilFaceState {
                        // Compare function is not equal.
                        // later in the pass we define the value to
                        // compare
                        compare: CompareFunction::NotEqual,
                        // we don't care about the stencil buffer after the color
                        // pass because it is cleared after the pass
                        fail_op: StencilOperation::Zero,
                        depth_fail_op: StencilOperation::Zero,
                        pass_op: StencilOperation::Zero,
                    },
                    // For the color pass we have backface culling enabled.
                    // Therefore we don't need a stencil test for the back of a face
                    back: StencilFaceState::IGNORE,
                    read_mask: !0,
                    write_mask: !0,
                },
                bias: DepthBiasState::default(),
            }),
            multiview: None,
            multisample,
        });

        pipeline
    }

    fn build_fans_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        msaa: &MSAA,
        globals_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let multisample = wgpu::MultisampleState {
            count: msaa.clone().into(),
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../../../../shader/path/fan.wgsl"
            ))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[globals_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[FanVertex::DESCRIPTION],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[],
            }),
            primitive: wgpu::PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: false,
                depth_compare: CompareFunction::Always,
                stencil: StencilState {
                    front: StencilFaceState {
                        compare: CompareFunction::Always,
                        fail_op: StencilOperation::Keep,
                        // See Kokojima resolution independent rendering paper.
                        // We simply invert the stencil value for each front and back face
                        depth_fail_op: StencilOperation::IncrementWrap,
                        pass_op: StencilOperation::IncrementWrap,
                    },
                    back: StencilFaceState {
                        compare: CompareFunction::Always,
                        fail_op: StencilOperation::Keep,
                        // For each back face we decrement the the stencil buffer at this
                        // fragment. Later in the color pass we just use fragments that have
                        // a stencil value other than zero
                        depth_fail_op: StencilOperation::DecrementWrap,
                        pass_op: StencilOperation::DecrementWrap,
                    },
                    read_mask: !0,
                    write_mask: !0,
                },
                bias: DepthBiasState::default(),
            }),
            multiview: None,
            multisample,
        });

        pipeline
    }

    fn build_segments_buffer_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bind-group-layout of segments storage buffer"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }

    fn build_segments_convex_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        msaa: &MSAA,
        segments_bind_group_layout: &wgpu::BindGroupLayout,
        globals_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let multisample = wgpu::MultisampleState {
            count: msaa.clone().into(),
            mask: !0,
            alpha_to_coverage_enabled: true,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../../../../shader/path/segment_convex.wgsl"
            ))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[segments_bind_group_layout, globals_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[SegmentVertex::DESCRIPTION],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[],
            }),
            primitive: wgpu::PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: false,
                depth_compare: CompareFunction::Always,
                stencil: StencilState {
                    front: StencilFaceState {
                        // We check if the current value is equal zero
                        compare: CompareFunction::Always,
                        // Else we clear the fragment
                        fail_op: StencilOperation::Zero,
                        depth_fail_op: StencilOperation::IncrementWrap,
                        // When the value is zero we increment the value
                        pass_op: StencilOperation::IncrementWrap,
                    },
                    // We do the same for the back face
                    back: StencilFaceState {
                        compare: CompareFunction::Always,
                        fail_op: StencilOperation::Zero,
                        depth_fail_op: StencilOperation::IncrementWrap,
                        pass_op: StencilOperation::IncrementWrap,
                    },
                    read_mask: !0,
                    write_mask: !0,
                },
                bias: DepthBiasState::default(),
            }),
            multiview: None,
            multisample,
        });

        pipeline
    }

    fn build_segments_concave_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        msaa: &MSAA,
        segments_bind_group_layout: &wgpu::BindGroupLayout,
        globals_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let multisample = wgpu::MultisampleState {
            count: msaa.clone().into(),
            mask: !0,
            alpha_to_coverage_enabled: true,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../../../../shader/path/segment_concave.wgsl"
            ))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[segments_bind_group_layout, globals_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[SegmentVertex::DESCRIPTION],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[],
            }),
            primitive: wgpu::PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: false,
                depth_compare: CompareFunction::Always,
                stencil: StencilState {
                    front: StencilFaceState {
                        // We check if the current value is equal zero
                        compare: CompareFunction::Greater,
                        // Else we clear the fragment
                        fail_op: StencilOperation::Zero,
                        depth_fail_op: StencilOperation::DecrementWrap,
                        // When the value is zero we increment the value
                        pass_op: StencilOperation::DecrementWrap,
                    },
                    // We do the same for the back face
                    back: StencilFaceState {
                        compare: CompareFunction::Greater,
                        fail_op: StencilOperation::Zero,
                        depth_fail_op: StencilOperation::DecrementWrap,
                        pass_op: StencilOperation::DecrementWrap,
                    },
                    read_mask: !0,
                    write_mask: !0,
                },
                bias: DepthBiasState::default(),
            }),
            multiview: None,
            multisample,
        });

        pipeline
    }

    fn build_triangle_fan_and_curve_triangles(
        path: &PathNode,
        fan_vertices: &mut Vec<vertex::FanVertex>,
        fan_indices: &mut Vec<u16>,
        segment_vertices: &mut Vec<vertex::SegmentVertex>,
        segment_indices: &mut Vec<u16>,
        segments: &mut Vec<primitive::PathSegment1>,
    ) {
        let n = path.segments.len();
        // Choose a pivot point for triangle fan
        let pivot = path.from;
        let pivot_index = fan_vertices.len();
        fan_vertices.push(vertex::FanVertex { pos: pivot });

        // Start point of a segment
        let mut p0 = pivot;
        for i in 0..n {
            let segment_index = segments.len() as u32;
            match &path.segments[i] {
                PathSegment::Linear { .. } => {}
                PathSegment::Arc { .. } => {}
                PathSegment::QuadraticBezier { to, param } => {
                    let index = segment_vertices.len();
                    segment_indices.push(index as u16);
                    segment_indices.push((index + 1) as u16);
                    segment_indices.push((index + 2) as u16);
                    segment_vertices.push(vertex::SegmentVertex {
                        pos: p0,
                        segment_index,
                    });
                    segment_vertices.push(vertex::SegmentVertex {
                        pos: *param,
                        segment_index,
                    });
                    segment_vertices.push(vertex::SegmentVertex {
                        pos: *to,
                        segment_index,
                    });
                    /*
                    segments.push(primitive::PathSegment1 {
                        param0: p0,
                        param1: *param,
                        param2: *to,
                        param3: [0.0, 0.0],
                    });*/
                }
                PathSegment::CubicBezier { to, params } => {
                    let index = segment_vertices.len();
                    segment_indices.push(index as u16);
                    segment_indices.push((index + 1) as u16);
                    segment_indices.push((index + 2) as u16);
                    segment_indices.push((index + 1) as u16);
                    segment_indices.push((index + 2) as u16);
                    segment_indices.push((index + 3) as u16);
                    segment_vertices.push(vertex::SegmentVertex {
                        pos: p0,
                        segment_index,
                    });
                    segment_vertices.push(vertex::SegmentVertex {
                        pos: params[0],
                        segment_index,
                    });
                    segment_vertices.push(vertex::SegmentVertex {
                        pos: params[1],
                        segment_index,
                    });
                    segment_vertices.push(vertex::SegmentVertex {
                        pos: *to,
                        segment_index,
                    });

                    segments.push(primitive::PathSegment1 {
                        param0: p0,
                        param1: params[0],
                        param2: params[1],
                        param3: *to,
                    });
                }
                PathSegment::CatmullRom => {}
            }
            let to0 = path.segments[i].to();
            if i < n - 1 {
                // Get next control point and the next of the next
                let to1 = path.segments[i + 1].to();

                // Add the indices for the new triangle
                let index = fan_vertices.len();
                fan_indices.push(pivot_index as u16);
                fan_indices.push(index as u16);
                fan_indices.push((index + 1) as u16);

                // Add the vertices of new triangle to the vertices
                fan_vertices.push(vertex::FanVertex { pos: *to0 });
                fan_vertices.push(vertex::FanVertex { pos: *to1 });
            }

            // Change p0 to the next control point
            p0 = *to0;
        }
    }

    pub(crate) fn record_stencil_pass<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if let Some(mount) = &self.mount {
            // Draw the triangle fans into the stencil buffer
            render_pass.set_pipeline(&self.fans_pipeline);
            render_pass.set_bind_group(0, &self.globals_bind_group, &[]);
            render_pass.set_index_buffer(mount.fan_index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.set_vertex_buffer(0, mount.fan_vertex_buffer.slice(..));
            render_pass.draw_indexed(0..mount.fan_index_count, 0, 0..1);

            // Change the pipeline so that the curve segments are drawn into the stencil buffer
            render_pass.set_pipeline(&self.segments_convex_pipeline);
            // We need to compare the current value with zero.
            render_pass.set_stencil_reference(0);
            // if the stencil buffer fragment is zero then we draw it.
            // Else we delete the current fragment
            render_pass.set_bind_group(0, &mount.segments_buffer_bind_group, &[]);
            render_pass.set_bind_group(1, &self.globals_bind_group, &[]);
            render_pass.set_index_buffer(mount.segment_index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.set_vertex_buffer(0, mount.segment_vertex_buffer.slice(..));
            render_pass.draw_indexed(0..mount.segment_index_count, 0, 0..1);

            render_pass.set_pipeline(&self.segments_concave_pipeline);
            render_pass.set_stencil_reference(0);
            render_pass.draw_indexed(0..mount.segment_index_count, 0, 0..1);
        }
    }

    pub(crate) fn record<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if let Some(mount) = &self.mount {
            render_pass.set_pipeline(&self.color_pipeline);
            render_pass.set_index_buffer(mount.color_index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.set_vertex_buffer(0, mount.color_vertex_buffer.slice(..));
            // Stencil reference is important in this pass
            // We throw away everything from the quad that is zero
            // or in other words we keep everything that is non zero
            render_pass.set_stencil_reference(0);

            // Draw the fan triangles
            render_pass.draw_indexed(0..mount.color_index_count, 0, 0..1);
        }
    }

    pub(crate) fn mount(&mut self, device: &wgpu::Device, paths: &Vec<PathNode>) {
        // Build the new data for vertex, index and storage buffers
        // required to render path segments on
        let mut fan_vertices = vec![];
        let mut fan_indices = vec![];
        let mut segment_vertices = vec![];
        let mut segment_indices = vec![];
        let mut segments = vec![];

        // Render the paths that are provided
        for path in paths {
            Self::build_triangle_fan_and_curve_triangles(
                path,
                &mut fan_vertices,
                &mut fan_indices,
                &mut segment_vertices,
                &mut segment_indices,
                &mut segments,
            );
        }

        // Create the necessary buffers
        let fan_vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Fan vertex Buffer"),
            contents: bytemuck::cast_slice(&fan_vertices),
            usage: BufferUsages::VERTEX,
        });
        let fan_index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Fan index Buffer"),
            contents: bytemuck::cast_slice(&fan_indices),
            usage: BufferUsages::INDEX,
        });
        let segment_vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Segment vertex Buffer"),
            contents: bytemuck::cast_slice(&segment_vertices),
            usage: BufferUsages::VERTEX,
        });
        let segment_index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Segment index Buffer"),
            contents: bytemuck::cast_slice(&segment_indices),
            usage: BufferUsages::INDEX,
        });
        let segments_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Segments storage Buffer"),
            contents: bytemuck::cast_slice(segments.as_slice()),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });
        let segments_buffer_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.segments_buffer_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: segments_buffer.as_entire_binding(),
            }],
            label: Some("Segments storage buffer bind group"),
        });
        //           +y
        //           |
        // -x -------|--------- +x
        //           |
        //          -y
        let vertices = vec![
            ColorVertex { pos: [-1.0, -1.0] },
            ColorVertex { pos: [-1.0, 1.0] },
            ColorVertex { pos: [1.0, -1.0] },
            ColorVertex { pos: [1.0, 1.0] },
        ];
        let color_vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Color Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });
        let indices = [0u16, 3, 1, 0, 2, 3];
        let color_index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Color Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });

        self.mount = Some(Mount {
            fan_vertex_buffer,
            fan_index_buffer,
            fan_index_count: fan_indices.len() as u32,
            segment_vertex_buffer,
            segment_index_buffer,
            segment_index_count: segment_indices.len() as u32,
            segments_buffer,
            segments_buffer_bind_group,
            color_vertex_buffer,
            color_index_buffer,
            color_index_count: indices.len() as u32,
        });
    }

    pub(crate) fn resize(&self, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration) {
        let globals = primitive::Globals {
            width_height: util::pack(config.width as u16, config.height as u16),
            aspect_ratio: config.width as f32 / config.height as f32,
        };
        queue.write_buffer(&self.globals_buffer, 0, bytemuck::cast_slice(&[globals]));
    }
}
