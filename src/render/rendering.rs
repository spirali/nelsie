use super::text::render_text;
use crate::model::{
    Color, Drawing, FontData, Node, NodeChild, NodeContent, NodeId, Span, Step, StyledLine,
    StyledText, TextAlign, TextStyle,
};
use crate::render::core::RenderConfig;
use crate::render::layout::{ComputedLayout, LayoutContext, Rectangle};

use std::collections::BTreeSet;

use resvg::tiny_skia;
use std::rc::Rc;
use std::sync::Arc;

use crate::render::counters::replace_counters;
use crate::render::image::render_image;
use crate::render::paths::{create_arrow, create_path, path_from_rect};

use svg2pdf::usvg;
use usvg::{Fill, Stroke, Tree};

pub(crate) struct RenderContext<'a> {
    config: &'a RenderConfig<'a>,
    z_level: i32,
    layout: &'a ComputedLayout,
    svg_node: &'a mut usvg::Group,
}

impl From<&Color> for usvg::Color {
    fn from(value: &Color) -> Self {
        let c: svgtypes::Color = value.into();
        usvg::Color::new_rgb(c.red, c.green, c.blue)
    }
}

fn draw_debug_frame(
    rect: &Rectangle,
    name: &str,
    font: &Arc<FontData>,
    color: &Color,
    svg_node: &mut usvg::Group,
) {
    let mut path = usvg::Path::new(Rc::new(tiny_skia::PathBuilder::from_rect(
        tiny_skia::Rect::from_xywh(rect.x, rect.y, rect.width.max(1.0), rect.height.max(1.0))
            .unwrap(),
    )));
    path.stroke = Some(Stroke {
        paint: usvg::Paint::Color(color.into()),
        dasharray: Some(vec![5.0, 2.5]),
        ..Default::default()
    });
    svg_node.children.push(usvg::Node::Path(Box::new(path)));
    let text = if name.is_empty() {
        format!("[{}x{}]", rect.width, rect.height)
    } else {
        format!("{} [{}x{}]", name, rect.width, rect.height)
    };
    let styled_text = StyledText {
        styled_lines: vec![StyledLine {
            spans: vec![Span {
                length: text.len() as u32,
                style_idx: 0,
            }],
            text,
        }],
        styles: vec![TextStyle {
            font: font.clone(),
            stroke: None,
            color: Some(color.clone()),
            size: 8.0,
            line_spacing: 0.0,
            italic: false,
            stretch: Default::default(),
            weight: 700,
            kerning: true,
        }],
        default_font_size: 8.0,
        default_line_spacing: 0.0,
    };
    svg_node.children.push(render_text(
        &styled_text,
        rect.x + 2.0,
        rect.y + 3.0,
        TextAlign::Start,
    ));
}

impl<'a> RenderContext<'a> {
    pub fn new(
        config: &'a RenderConfig<'a>,
        z_level: i32,
        layout: &'a ComputedLayout,
        svg_node: &'a mut usvg::Group,
    ) -> RenderContext<'a> {
        RenderContext {
            config,
            z_level,
            layout,
            svg_node,
        }
    }

    fn render_helper(&mut self, mut step: Step, node: &Node) {
        // active is before step replacement!
        if !node.active.at_step(step) {
            return;
        }
        if let Some(s) = node.replace_steps.get(&step) {
            step = *s;
        }
        if !node.show.at_step(step) {
            return;
        }
        let is_current_z_level = *node.z_level.at_step(step) == self.z_level;
        if is_current_z_level {
            if let Some(color) = &node.bg_color.at_step(step) {
                let rect = &self.layout.node_layout(node.node_id).unwrap().rect;
                let border_radius = *node.border_radius.at_step(step);
                if let Some(p) = path_from_rect(rect, border_radius) {
                    let mut path = usvg::Path::new(Rc::new(p));
                    path.fill = Some(Fill {
                        paint: usvg::Paint::Color(color.into()),
                        opacity: color.opacity(),
                        rule: Default::default(),
                    });
                    self.svg_node
                        .children
                        .push(usvg::Node::Path(Box::new(path)));
                }
            }

            if let Some(content) = &node.content {
                let rect = &self.layout.node_layout(node.node_id).unwrap().rect;
                match content {
                    NodeContent::Text(text) => {
                        let mut t = text.text_style_at_step(step);
                        if text.parse_counters {
                            // Here we do not "step" but "self.config.step" as we want to escape "replace_steps"
                            // for counters
                            replace_counters(
                                self.config.counter_values,
                                &mut t,
                                self.config.slide_idx,
                                self.config.step,
                            );
                        }
                        self.svg_node.children.push(render_text(
                            &t,
                            match text.text_align {
                                TextAlign::Start => rect.x,
                                TextAlign::Center => rect.x + rect.width / 2.0,
                                TextAlign::End => rect.x + rect.width,
                            },
                            rect.y,
                            text.text_align,
                        ))
                    }
                    NodeContent::Image(image) => render_image(
                        step,
                        image,
                        rect,
                        self.svg_node,
                        &self.config.resources.font_db,
                    ),
                }
            }

            if let Some(color) = &node.debug_layout {
                let rect = &self.layout.node_layout(node.node_id).unwrap().rect;
                draw_debug_frame(
                    rect,
                    &node.name,
                    self.config.default_font,
                    color,
                    self.svg_node,
                );
            }
        }

        for child in &node.children {
            match child {
                NodeChild::Node(node) => self.render_helper(step, node),
                NodeChild::Draw(draw) => {
                    if is_current_z_level {
                        self.draw(step, node.node_id, draw)
                    }
                }
            }
        }
    }

    fn draw(&mut self, step: Step, parent_id: NodeId, drawing: &Drawing) {
        for path in drawing.paths.at_step(step) {
            if let Some(usvg_path) = create_path(self.layout, parent_id, path) {
                self.svg_node
                    .children
                    .push(usvg::Node::Path(Box::new(usvg_path)));
            }
            if let Some(usvg_path) = create_arrow(self.layout, parent_id, path, true) {
                self.svg_node
                    .children
                    .push(usvg::Node::Path(Box::new(usvg_path)));
            }
            if let Some(usvg_path) = create_arrow(self.layout, parent_id, path, false) {
                self.svg_node
                    .children
                    .push(usvg::Node::Path(Box::new(usvg_path)));
            }
        }
    }

    pub(crate) fn render_to_svg(mut self, step: Step, node: &Node) {
        self.render_helper(step, node);
    }
}

pub(crate) fn render_to_svg_tree(render_cfg: &RenderConfig) -> usvg::Tree {
    log::debug!("Creating layout");
    let layout_builder = LayoutContext::new(render_cfg);
    let layout = layout_builder.compute_layout(render_cfg.slide, render_cfg.step);

    log::debug!("Layout {:?}", layout);

    let mut z_levels = BTreeSet::new();
    render_cfg.slide.node.collect_z_levels(&mut z_levels);

    log::debug!("Rendering to svg");
    let mut root_svg_node = usvg::Group::default();

    let slide = &render_cfg.slide;
    let mut path = usvg::Path::new(Rc::new(tiny_skia::PathBuilder::from_rect(
        tiny_skia::Rect::from_xywh(0.0, 0.0, slide.width, slide.height).unwrap(),
    )));
    path.fill = Some(Fill {
        paint: usvg::Paint::Color((&slide.bg_color).into()),
        opacity: slide.bg_color.opacity(),
        rule: Default::default(),
    });
    root_svg_node
        .children
        .push(usvg::Node::Path(Box::new(path)));

    for z_level in z_levels {
        let render_ctx = RenderContext::new(render_cfg, z_level, &layout, &mut root_svg_node);
        render_ctx.render_to_svg(render_cfg.step, &render_cfg.slide.node);
    }
    let size = usvg::Size::from_wh(render_cfg.slide.width, render_cfg.slide.height).unwrap();

    Tree {
        size,
        view_box: usvg::ViewBox {
            rect: size.to_non_zero_rect(0.0, 0.0),
            aspect: usvg::AspectRatio::default(),
        },
        root: root_svg_node,
    }
}
