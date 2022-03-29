use crate::Node;
use crate::node::base::BaseNode;
use crate::renderer::wgpu::pipeline::image_pipeline::ImagePipeline;
use crate::renderer::wgpu::pipeline::{image_pipeline, rect_pipeline};
use crate::renderer::wgpu::pipeline::rect_pipeline::RectPipeline;
use crate::util::{Extent, Flags, Offset, Rect};
use async_recursion::async_recursion;

pub struct RenderJob {
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub(crate) surface: wgpu::Surface,
    pub(crate) rect_pipeline: RectPipeline,
    pub(crate) image_pipeline: ImagePipeline
}
impl RenderJob {
    pub(crate) fn new(device: &wgpu::Device, config: wgpu::SurfaceConfiguration, surface: wgpu::Surface) -> Self {
        let rect_pipeline = RectPipeline::new(device, &config);
        let image_pipeline = ImagePipeline::new(device, &config);
        RenderJob {
            config,
            surface,
            rect_pipeline,
            image_pipeline
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
    async fn flatten(root: &Rect, parent: &Rect, node: &Node, rects: &mut Vec<rect_pipeline::Instance>, images: &mut Vec<image_pipeline::Instance>) {
        match node {
            Node::Rectangle(base) => {
                let r = Self::rect(parent, base);
                rects.push(rect_pipeline::Instance {
                    rect: Self::rect(parent, base).norm(root),
                    color: base.background.as_raw()
                })
            },
            Node::Border(base, b) => {
                Self::flatten(root, parent, b.node(), rects, images).await;
            },
            Node::Composition(_, c) => {
                for node in c.layers() {
                    Self::flatten(root, parent, node, rects, images).await;
                }
            },
            Node::Image(_, i) => {
                todo!()
            },
            Node::Text(_, t) => {
                todo!()
            },
            Node::Component(_, c) => {
                let node = c.node().await;
                Self::flatten(root, parent, &node, rects, images).await;
            }
        }
    }

    pub(crate) async fn mount(&mut self, device: &wgpu::Device, node: &Node) {
        let mut rects = vec![];
        let mut images = vec![];
        let root = Rect::new(0, 0, self.config.width, self.config.height);
        Self::flatten(&root, &root, node, &mut rects, &mut images).await;
        self.rect_pipeline.mount(device, &rects);
        //self.image_pipeline.mount(device, &images);
    }

    pub(crate) fn record<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.rect_pipeline.record(render_pass);
        self.image_pipeline.record(render_pass);
    }

    pub(crate) fn resize(&mut self, device: &wgpu::Device, size: Extent) {
        self.config.width = size.width.max(1);
        self.config.height = size.height.max(1);
        self.surface.configure(device, &self.config);
    }
}