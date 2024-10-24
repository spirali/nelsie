use crate::model::{
    Drawing, FontData, Node, NodeChild, NodeContent, NodeId, Span, Step, StyledLine, StyledText,
    TextAlign, TextStyle,
};
use crate::render::layout::{compute_layout, ComputedLayout};
use crate::render::RenderConfig;

use std::collections::BTreeSet;

use std::sync::Arc;

use crate::render::image::render_image_to_canvas;
use crate::render::paths::{draw_item_from_rect, eval_path};

use crate::common::{Color, DrawItem, DrawRect, Rectangle, Stroke};
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
        fill_color: None,
        stroke: Some(Stroke {
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
    let styled_text = StyledText {
        styled_lines: vec![StyledLine {
            spans: vec![Span {
                length: text.len() as u32,
                style_idx: None,
            }],
            text,
        }],
        main_style: TextStyle {
            font: font.clone(),
            color: *color,
            size: 8.0,
            line_spacing: 0.0,
            italic: false,
            stretch: Default::default(),
            weight: 700,
            underline: false,
            line_through: false,
        },
        styles: Vec::new(),
        anchors: Default::default(),
    };
    canvas.add_text(
        Arc::new(RenderedText::render(
            text_context,
            &styled_text,
            TextAlign::Start,
        )),
        rect.x + 2.0,
        rect.y + 3.0,
    );
}

fn draw_debug_step(
    rect: &Rectangle,
    step: &Step,
    font_name: &str,
    font_size: f32,
    canvas: &mut Canvas,
) {
    todo!()
    /*let mut xml = SimpleXmlWriter::new();
    xml.begin("rect");
    xml.attr("x", rect.x);
    xml.attr("y", rect.y);
    xml.attr("width", rect.width);
    xml.attr("height", rect.height);
    xml.attr("fill", "black");
    xml.end("rect");
    xml.begin("text");
    xml.begin("tspan");
    xml.attr("x", 10);
    xml.attr("y", rect.y + font_size);
    xml.attr("fill", "white");
    xml.attr("font-size", font_size);
    xml.attr("font-family", font_name);
    xml.text(&step.to_string());
    xml.end("tspan");
    xml.end("text");
    canvas.add_item(CanvasItem::SvgChunk(xml.into_string()));*/
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
                let item = draw_item_from_rect(rect, border_radius, None, Some(*color));
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
            &Rectangle::new(
                0.0,
                render_cfg.slide.height,
                render_cfg.slide.width,
                DEBUG_STEP_FONT_SIZE * 1.25,
            ),
            render_cfg.step,
            &render_cfg.default_font.family_name,
            DEBUG_STEP_FONT_SIZE,
            &mut canvas,
        );
    }

    canvas
}
