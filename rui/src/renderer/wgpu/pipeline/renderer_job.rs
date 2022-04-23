use std::marker::PhantomData;
use std::sync::Arc;
use crate::{Backend, Node};
use crate::node::base::BaseNode;
use crate::renderer::wgpu::pipeline::image_pipeline::ImagePipeline;
use crate::renderer::wgpu::pipeline::rect_pipeline::RectPipeline;
use crate::util::{Flags, PathSegment, Rect};
use async_recursion::async_recursion;
use rui_util::Extent;
use crate::renderer::wgpu::pipeline::path_pipeline::PathPipeline;
use crate::renderer::wgpu::primitive;
use crate::surface::Surface;

pub struct RenderJob<B> where B: Backend {
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub(crate) surface_adapter: Arc<Surface>,
    pub(crate) surface: wgpu::Surface,
    pub(crate) rect_pipeline: RectPipeline,
    pub(crate) image_pipeline: ImagePipeline,
    pub(crate) path_pipeline: PathPipeline,
    _b: PhantomData<B>
}
impl<B> RenderJob<B> where B: Backend {
    pub(crate) fn new(device: &wgpu::Device, config: wgpu::SurfaceConfiguration, surface_adapter: Arc<Surface>, surface: wgpu::Surface) -> Self {
        let rect_pipeline = RectPipeline::new(device, &config);
        let image_pipeline = ImagePipeline::new(device, &config);
        let path_pipeline = PathPipeline::new(device, &config);
        RenderJob {
            config,
            surface_adapter,
            surface,
            rect_pipeline,
            image_pipeline,
            path_pipeline,
            _b: PhantomData
        }
    }

    //Calculate rect of primitive element
    fn rect(parent: &Rect, base: &BaseNode) -> Rect {
        //debug_assert!(parent.intersect(&base.bounding_rect).is_some());
        let mut width = base.bounding_rect.extent.width.clamp(0, (parent.extent.width as i32 - base.bounding_rect.offset.x) as u32);
        let mut height = base.bounding_rect.extent.height.clamp(0, (parent.extent.height as i32 - base.bounding_rect.offset.y) as u32);
        if base.flags.test(Flags::AUTO_WIDTH) {
            width = (parent.extent.width as i32 - base.bounding_rect.offset.x).clamp(0, i32::MAX) as u32;
        }
        if base.flags.test(Flags::AUTO_HEIGHT) {
            height = (parent.extent.height as i32 - base.bounding_rect.offset.y).clamp(0, i32::MAX) as u32;
        }
        Rect {
            offset: base.bounding_rect.offset.clone(),
            extent: Extent {
                width,
                height
            }
        }
    }

    // For now a simple recursive variant
    // TODO: In the future replace this method through an
    // iterative method to reduce stack size
    // Or event optimize node graph to completely get rid of it

    #[async_recursion]
    async fn flatten(root: &Rect, parent: &Rect, node: &mut Node, rects: &mut Vec<primitive::Rect>, images: &mut Vec<primitive::Image>, paths: &mut Vec<primitive::Path>) {
        match node {
            Node::Rectangle(base) => {
                rects.push(primitive::Rect {
                    rect: Self::rect(parent, base).norm(root),
                    color: base.background.as_raw(),
                    radii: base.border_radii
                })
            },
            Node::Border(base, b) => {
                Self::flatten(root, parent, b.node_mut(), rects, images, paths).await;
            },
            Node::Path(base, p) => {
                let mut segments = Vec::with_capacity(p.segments().len());
                let mut from = p.from();
                for s in p.segments() {
                    segments.push(match s {
                        PathSegment::Linear { to } => {
                            let start = *from;
                            from = to;
                            primitive::PathSegment {
                                typ: primitive::PathSegment::LINEAR,
                                flags: 0,
                                param0: start,
                                param1: *to,
                                param2: [0.0, 0.0],
                                param3: [0.0, 0.0]
                            }
                        }
                        PathSegment::Arc { to, radii } => {
                            panic!()
                        }
                        PathSegment::QuadraticBezier { to, param } => {
                            panic!()
                        }
                        PathSegment::CubicBezier { to, params } => {
                            let start = *from;
                            from = to;
                            primitive::PathSegment {
                                typ: primitive::PathSegment::CUBIC_BEZIER,
                                flags: 0,
                                param0: start,
                                param1: params[0],
                                param2: params[1],
                                param3: *to
                            }
                        }
                        PathSegment::CatmullRom => panic!()
                    });
                }
                paths.push(primitive::Path {
                    rect: Self::rect(parent, base).norm(root),
                    color: base.background.as_raw(),
                    segments
                })
            },
            Node::Composition(_, c) => {
                for node in c.layers_mut() {
                    Self::flatten(root, parent, node, rects, images, paths).await;
                }
            },
            Node::Image(base, i) => {
                images.push(primitive::Image {
                    instance: primitive::Instance {
                        rect: Self::rect(parent, base).norm(root),
                        color: base.background.as_raw(),
                        radii: base.border_radii
                    },
                    resource: i.resource().clone()
                })
            },
            Node::Text(_, t) => {
                todo!()
            },
            Node::Component(_, c) => {
                let mut node = c.node().await;
                Self::flatten(root, parent, &mut node, rects, images, paths).await;
            }
        }
    }

    pub(crate) async fn mount(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, node: &mut Node) {
        let mut rects = vec![];
        let mut images = vec![];
        let mut paths = vec![];
        let root = Rect::new(0, 0, self.config.width, self.config.height);
        Self::flatten(&root, &root, node, &mut rects, &mut images, &mut paths).await;
        self.rect_pipeline.mount(device, &rects);
        self.image_pipeline.mount(device, queue, &images).await;
        self.path_pipeline.mount(device, &paths);
    }

    pub(crate) fn record<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.rect_pipeline.record(render_pass);
        self.image_pipeline.record(render_pass);
        self.path_pipeline.record(render_pass);
    }

    pub(crate) fn resize(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, size: Extent) {
        self.config.width = size.width.max(1);
        self.config.height = size.height.max(1);
        self.rect_pipeline.resize(queue, &self.config);
        self.image_pipeline.resize(queue, &self.config);
        self.path_pipeline.resize(queue, &self.config);
        self.surface.configure(device, &self.config);
    }
}