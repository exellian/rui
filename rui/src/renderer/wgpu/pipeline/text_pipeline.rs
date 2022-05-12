use crate::font::fallback_fonts::{FallbackFonts, FontFamily, FontStyle};
use crate::renderer::wgpu::primitive;
use crate::Node;
use alloc::rc::Rc;
use glyph_brush::Color;
use std::cell::{Cell, RefMut};
use wgpu::util::StagingBelt;
use wgpu::{
    CommandEncoder, Device, RenderPass, RenderPassDepthStencilAttachment, Surface, TextureView,
};
use wgpu_glyph::{GlyphBrush, GlyphBrushBuilder, Section};
use wgpu_types::{CommandEncoderDescriptor, SurfaceConfiguration};

pub struct TextPipeline {
    brush: GlyphBrush<()>,
    staging_belt: StagingBelt,
}

impl TextPipeline {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let brush = GlyphBrushBuilder::using_font(FallbackFonts::get_font(
            FontFamily::SansSerif,
            FontStyle::Regular,
        ))
        .build(device, config.format);

        let staging_belt = StagingBelt::new(2048);
        TextPipeline {
            brush,
            staging_belt,
        }
    }

    pub fn mount(&mut self, device: &wgpu::Device, texts: &Vec<Node>) {
        for node in texts {
            if let Node::Text(base, text) = node {
                eprintln!("inside text node mount");
                let font_id = self.brush.add_font(text.font_resource());
                let renderable_text: wgpu_glyph::Text = wgpu_glyph::Text::new(text.text())
                    .with_color([0.7, 0.5, 0.9, 0.3])
                    .with_scale(text.font_size())
                    .with_font_id(font_id);
                let section = Section::default().with_text(vec![renderable_text]);
                self.brush.queue(section);
            }
        }
    }

    pub fn draw_queued(
        &mut self,
        device: &Device,
        staging_belt: &mut StagingBelt,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        width: u32,
        height: u32,
    ) {
        self.brush
            .draw_queued(device, staging_belt, encoder, view, width, height)
            .expect("Could not draw enqueued texts!");
    }

    pub fn resize(&mut self, config: &SurfaceConfiguration) {}
}
