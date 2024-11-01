use crate::model::{
    Drawing, FontData, Node, NodeChild, NodeContent, NodeId, Step, StyledText, TextStyle,
};
use crate::render::layout::{compute_layout, ComputedLayout};
use crate::render::RenderConfig;

use std::collections::BTreeSet;

use resvg::usvg::{FontStretch, PositiveF32};
use std::sync::Arc;

use crate::render::image::render_image_to_canvas;
use crate::render::paths::{draw_item_from_rect, eval_path};

use crate::common::{Color, DrawItem, DrawRect, FillAndStroke, Rectangle, Stroke};
use crate::render::canvas::{Canvas, Link};
use crate::render::text::{RenderedText, TextContext};
use svg2pdf::usvg;

pub(crate) struct RenderContext<'a> {
    z_level: i32,
    layout: &'a ComputedLayout,
    canvas: &'a mut Canvas,
}

impl From<&Color> for usvg::Color {
    fn from(value: &Color) -> Self {
        let c: svgtypes::Color = value.into();
        usvg::Color::new_rgb(c.red, c.green, c.blue)
    }
}

const DEBUG_STEP_FONT_SIZE: f32 = 48.0;

fn draw_debug_frame(
    text_context: &mut TextContext,
    rect: &Rectangle,
    name: &str,
    font: &Arc<FontData>,
    color: &Color,
    canvas: &mut Canvas,
) {
    let draw_item = DrawItem::Rect(DrawRect {
        rectangle: Rectangle {
            x: rect.x,
            y: rect.y,
            width: rect.width.max(1.0),
            height: rect.height.max(1.0),
        },
        fill_and_stroke: FillAndStroke::new_stroke(Stroke {
            color: *color,
            width: 1.0,
            dash_array: Some(vec![5.0, 2.5]),
            dash_offset: 0.0,
        }),
    });
    canvas.add_draw_item(draw_item);

    let text = if name.is_empty() {
        format!("[{}x{}]", rect.width, rect.height)
    } else {
        format!("{} [{}x{}]", name, rect.width, rect.height)
    };
    let styled_text = StyledText::new_simple_text(
        text,
        TextStyle {
            font: font.clone(),
            color: *color,
            size: PositiveF32::new(8.0).unwrap(),
            line_spacing: PositiveF32::ZERO,
            italic: false,
            stretch: FontStretch::Normal,
            weight: 700,
            underline: false,
            line_through: false,
        },
    );
    canvas.add_text(
        Arc::new(RenderedText::render(text_context, &styled_text)),
        rect.x + 2.0,
        rect.y + 3.0,
    );
}

fn draw_debug_step(
    text_context: &mut TextContext,
    rect: &Rectangle,
    step: &Step,
    font: &Arc<FontData>,
    font_size: PositiveF32,
    canvas: &mut Canvas,
) {
    canvas.add_draw_item(DrawItem::Rect(DrawRect {
        rectangle: rect.clone(),
        fill_and_stroke: FillAndStroke::new_fill(Color::new(svgtypes::Color::black())),
    }));
    let text = step.to_string();
    let styled_text = StyledText::new_simple_text(
        text,
        TextStyle {
            font: font.clone(),
            color: Color::new(svgtypes::Color::white()),
            size: font_size,
            line_spacing: PositiveF32::ZERO,
            italic: false,
            stretch: FontStretch::Normal,
            weight: 400,
            underline: false,
            line_through: false,
        },
    );
    canvas.add_text(
        Arc::new(RenderedText::render(text_context, &styled_text)),
        rect.x + 10.0,
        rect.y + font_size.get() / 2.0 * 1.25,
    );
}

impl<'a> RenderContext<'a> {
    pub fn new(
        z_level: i32,
        layout: &'a ComputedLayout,
        canvas: &'a mut Canvas,
    ) -> RenderContext<'a> {
        RenderContext {
            z_level,
            layout,
            canvas,
        }
    }

    fn render_helper(&mut self, config: &mut RenderConfig, step: &Step, node: &Node) {
        // active is before step replacement!
        if !node.active.at_step(step) {
            return;
        }
        let step = node.replace_steps.get(step).unwrap_or(step);

        if !node.show.at_step(step) {
            return;
        }
        let is_current_z_level = *node.z_level.at_step(step) == self.z_level;
        if is_current_z_level {
            if let Some(color) = &node.bg_color.at_step(step) {
                let rect = &self.layout.node_layout(node.node_id).unwrap().rect;
                let border_radius = *node.border_radius.at_step(step);
                let item =
                    draw_item_from_rect(rect, border_radius, FillAndStroke::new_fill(*color));
                self.canvas.add_draw_item(item);
            }

            if let Some(content) = &node.content {
                let rect = &self.layout.node_layout(node.node_id).unwrap().rect;
                match content {
                    NodeContent::Text(_) => {
                        self.canvas.add_text(
                            config.text_cache.get(node.node_id).unwrap().clone(),
                            rect.x,
                            rect.y,
                        );
                    }
                    NodeContent::Image(image) => {
                        render_image_to_canvas(image, step, rect, self.canvas)
                    }
                }
            }

            if let Some(url) = &node.url.at_step(step) {
                let rect = &self.layout.node_layout(node.node_id).unwrap().rect;
                self.canvas.add_link(Link::new(rect.clone(), url.clone()));
            }

            if let Some(color) = &node.debug_layout {
                let rect = &self.layout.node_layout(node.node_id).unwrap().rect;
                draw_debug_frame(
                    &mut config.thread_resources.text_context,
                    rect,
                    &node.name,
                    config.default_font,
                    color,
                    self.canvas,
                );
            }
        }

        for child in &node.children {
            match child {
                NodeChild::Node(node) => self.render_helper(config, step, node),
                NodeChild::Draw(draw) => {
                    if is_current_z_level {
                        self.draw(step, node.node_id, draw)
                    }
                }
            }
        }
    }

    fn draw(&mut self, step: &Step, parent_id: NodeId, drawing: &Drawing) {
        for path in drawing.paths.at_step(step) {
            eval_path(self.canvas, path, self.layout, parent_id)
        }
    }

    pub(crate) fn render_to_canvas(mut self, config: &mut RenderConfig) {
        self.render_helper(config, config.step, &config.slide.node);
    }
}

pub(crate) fn render_to_canvas(render_cfg: &mut RenderConfig) -> Canvas {
    log::debug!("Creating layout");
    let layout = compute_layout(render_cfg, render_cfg.step);

    log::debug!("Layout {:?}", layout);

    let mut z_levels = BTreeSet::new();
    render_cfg.slide.node.collect_z_levels(&mut z_levels);

    log::debug!("Rendering to canvas");
    let mut canvas = Canvas::new(
        render_cfg.slide.width,
        render_cfg.slide.height
            + if render_cfg.slide.debug_steps {
                DEBUG_STEP_FONT_SIZE * 1.25
            } else {
                0.0
            },
        render_cfg.slide.bg_color,
    );

    for z_level in z_levels {
        let render_ctx = RenderContext::new(z_level, &layout, &mut canvas);
        render_ctx.render_to_canvas(render_cfg);
    }

    if render_cfg.slide.debug_steps {
        draw_debug_step(
            &mut render_cfg.thread_resources.text_context,
            &Rectangle::new(
                0.0,
                render_cfg.slide.height,
                render_cfg.slide.width,
                DEBUG_STEP_FONT_SIZE * 1.25,
            ),
            render_cfg.step,
            render_cfg.default_font,
            PositiveF32::new(DEBUG_STEP_FONT_SIZE).unwrap(),
            &mut canvas,
        );
    }

    canvas
}
