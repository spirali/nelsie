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

use crate::render::image::render_image;
use crate::render::paths::{create_arrow, create_path, path_from_rect};
use crate::render::Resources;
use usvg::Fill;
use usvg_tree::Stroke;

pub(crate) struct RenderContext<'a> {
    resources: &'a Resources,
    z_level: i32,
    layout: &'a ComputedLayout,
    svg_node: usvg::Node,
    default_font: &'a Arc<FontData>,
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
    svg_node: &usvg::Node,
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
    svg_node.append(usvg::Node::new(usvg::NodeKind::Path(path)));
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
            weight: 400,
            kerning: true,
        }],
        default_font_size: 8.0,
        default_line_spacing: 0.0,
    };
    svg_node.append(render_text(
        &styled_text,
        rect.x + 2.0,
        rect.y + 3.0,
        TextAlign::Start,
    ));
}

impl<'a> RenderContext<'a> {
    pub fn new(
        global_res: &'a Resources,
        z_level: i32,
        layout: &'a ComputedLayout,
        svg_node: usvg::Node,
        default_font: &'a Arc<FontData>,
    ) -> Self {
        RenderContext {
            resources: global_res,
            z_level,
            layout,
            svg_node,
            default_font,
        }
    }

    fn render_helper(&self, mut step: Step, node: &Node) {
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
        if *node.z_level.at_step(step) == self.z_level {
            if let Some(color) = &node.bg_color.at_step(step) {
                let rect = &self.layout.node_layout(node.node_id).unwrap().rect;
                let border_radius = *node.border_radius.at_step(step);
                let mut path = usvg::Path::new(Rc::new(path_from_rect(rect, border_radius)));
                path.fill = Some(Fill {
                    paint: usvg::Paint::Color(color.into()),
                    opacity: color.opacity(),
                    rule: Default::default(),
                });
                self.svg_node
                    .append(usvg::Node::new(usvg::NodeKind::Path(path)));
            }

            if let Some(content) = &node.content.at_step(step) {
                let rect = &self.layout.node_layout(node.node_id).unwrap().rect;
                match content {
                    NodeContent::Text(text) => self.svg_node.append(render_text(
                        &text.text_style_at_step(step),
                        match text.text_align {
                            TextAlign::Start => rect.x,
                            TextAlign::Center => rect.x + rect.width / 2.0,
                            TextAlign::End => rect.x + rect.width,
                        },
                        rect.y,
                        text.text_align,
                    )),
                    NodeContent::Image(image) => {
                        render_image(step, image, rect, &self.svg_node, &self.resources.font_db)
                    }
                }
            }

            if let Some(color) = &node.debug_layout {
                let rect = &self.layout.node_layout(node.node_id).unwrap().rect;
                draw_debug_frame(rect, &node.name, self.default_font, color, &self.svg_node);
            }
        }

        for child in &node.children {
            match child {
                NodeChild::Node(node) => self.render_helper(step, node),
                NodeChild::Draw(draw) => self.draw(step, node.node_id, draw),
            }
        }
    }

    fn draw(&self, step: Step, parent_id: NodeId, drawing: &Drawing) {
        for path in drawing.paths.at_step(step) {
            if let Some(usvg_path) = create_path(self.layout, parent_id, path) {
                self.svg_node
                    .append(usvg::Node::new(usvg::NodeKind::Path(usvg_path)));
            }
            if let Some(usvg_path) = create_arrow(self.layout, parent_id, path, true) {
                self.svg_node
                    .append(usvg::Node::new(usvg::NodeKind::Path(usvg_path)));
            }
            if let Some(usvg_path) = create_arrow(self.layout, parent_id, path, false) {
                self.svg_node
                    .append(usvg::Node::new(usvg::NodeKind::Path(usvg_path)));
            }
        }
    }

    pub(crate) fn render_to_svg(self, step: Step, node: &Node) {
        self.render_helper(step, node);
    }
}

pub(crate) fn render_to_svg_tree(render_cfg: &RenderConfig) -> usvg_tree::Tree {
    log::debug!("Creating layout");
    let layout_builder = LayoutContext::new(render_cfg.resources);
    let layout = layout_builder.compute_layout(render_cfg.slide, render_cfg.step);

    log::debug!("Layout {:?}", layout);

    let mut z_levels = BTreeSet::new();
    render_cfg.slide.node.collect_z_levels(&mut z_levels);

    log::debug!("Rendering to svg");
    let root_svg_node = usvg::Node::new(usvg::NodeKind::Group(usvg::Group::default()));

    let slide = &render_cfg.slide;
    let mut path = usvg::Path::new(Rc::new(tiny_skia::PathBuilder::from_rect(
        tiny_skia::Rect::from_xywh(0.0, 0.0, slide.width, slide.height).unwrap(),
    )));
    path.fill = Some(Fill {
        paint: usvg::Paint::Color((&slide.bg_color).into()),
        opacity: slide.bg_color.opacity(),
        rule: Default::default(),
    });
    root_svg_node.append(usvg::Node::new(usvg::NodeKind::Path(path)));

    for z_level in z_levels {
        let render_ctx = RenderContext::new(
            render_cfg.resources,
            z_level,
            &layout,
            root_svg_node.clone(),
            render_cfg.default_font,
        );
        render_ctx.render_to_svg(render_cfg.step, &render_cfg.slide.node);
    }
    let size = usvg::Size::from_wh(render_cfg.slide.width, render_cfg.slide.height).unwrap();

    usvg_tree::Tree {
        size,
        view_box: usvg::ViewBox {
            rect: size.to_non_zero_rect(0.0, 0.0),
            aspect: usvg::AspectRatio::default(),
        },
        root: root_svg_node,
    }
}
