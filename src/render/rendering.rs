use super::text::render_text;
use crate::model::{
    Color, Drawing, Node, NodeChild, NodeContent, NodeId, Span, Step, StyledLine, StyledText,
    TextStyle,
};
use crate::render::core::RenderConfig;
use crate::render::layout::{ComputedLayout, LayoutContext, Rectangle};

use std::collections::BTreeSet;

use resvg::tiny_skia;
use std::rc::Rc;
use std::sync::Arc;

use crate::render::image::render_image;
use crate::render::paths::create_path;
use crate::render::Resources;
use usvg::Fill;
use usvg_tree::Stroke;

pub(crate) struct RenderContext<'a> {
    resources: &'a Resources,
    step: Step,
    z_level: i32,
    layout: &'a ComputedLayout,
    svg_node: usvg::Node,
    default_font_name: &'a Arc<String>,
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
    font_name: &Arc<String>,
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
        styled_lines: &[StyledLine {
            spans: vec![Span {
                length: text.len() as u32,
                style_idx: 0,
            }],
            text,
        }],
        styles: vec![TextStyle {
            font_family: font_name.clone(),
            color: color.clone(),
            size: 8.0,
            line_spacing: 0.0,
            italic: false,
            stretch: Default::default(),
            weight: 400,
        }],
        default_font_size: 8.0,
        default_line_spacing: 0.0,
    };
    svg_node.append(render_text(&styled_text, rect.x + 2.0, rect.y + 3.0));
}

impl<'a> RenderContext<'a> {
    pub fn new(
        global_res: &'a Resources,
        step: Step,
        z_level: i32,
        layout: &'a ComputedLayout,
        svg_node: usvg::Node,
        default_font_name: &'a Arc<String>,
    ) -> Self {
        RenderContext {
            resources: global_res,
            step,
            z_level,
            layout,
            svg_node,
            default_font_name,
        }
    }

    fn render_helper(&self, node: &Node) {
        if !node.show.at_step(self.step) {
            return;
        }
        if *node.z_level.at_step(self.step) == self.z_level {
            if let Some(color) = &node.bg_color.at_step(self.step) {
                let rect = self.layout.rect(node.node_id).unwrap();
                let mut path = usvg::Path::new(Rc::new(tiny_skia::PathBuilder::from_rect(
                    tiny_skia::Rect::from_xywh(rect.x, rect.y, rect.width, rect.height).unwrap(),
                )));
                path.fill = Some(Fill {
                    paint: usvg::Paint::Color(color.into()),
                    opacity: color.opacity(),
                    rule: Default::default(),
                });
                self.svg_node
                    .append(usvg::Node::new(usvg::NodeKind::Path(path)));
            }

            if let Some(content) = &node.content.at_step(self.step) {
                let rect = self.layout.rect(node.node_id).unwrap();
                match content {
                    NodeContent::Text(text) => self.svg_node.append(render_text(
                        &text.text_style_at_step(self.step),
                        rect.x,
                        rect.y,
                    )),
                    NodeContent::Image(image) => render_image(
                        self.step,
                        image,
                        rect,
                        &self.svg_node,
                        &self.resources.font_db,
                    ),
                }
            }

            if let Some(color) = &node.debug_layout {
                let rect = self.layout.rect(node.node_id).unwrap();
                draw_debug_frame(
                    rect,
                    &node.name,
                    self.default_font_name,
                    color,
                    &self.svg_node,
                );
            }
        }

        for child in &node.children {
            match child {
                NodeChild::Node(node) => self.render_helper(node),
                NodeChild::Draw(draw) => self.draw(node.node_id, draw),
            }
        }
    }

    fn draw(&self, parent_id: NodeId, drawing: &Drawing) {
        for path in drawing.paths.at_step(self.step) {
            if let Some(usvg_path) = create_path(self.layout, parent_id, path) {
                self.svg_node
                    .append(usvg::Node::new(usvg::NodeKind::Path(usvg_path)));
            }
        }
    }

    pub(crate) fn render_to_svg(self, node: &Node) {
        self.render_helper(node);
    }
}

pub(crate) fn render_to_svg_tree(render_cfg: &RenderConfig) -> usvg_tree::Tree {
    log::debug!("Creating layout");
    let layout_builder = LayoutContext::new(render_cfg.resources, render_cfg.step);
    let layout = layout_builder.compute_layout(render_cfg.slide);

    log::debug!("Layout {:?}", layout);

    let mut z_levels = BTreeSet::new();
    render_cfg.slide.node.collect_z_levels(&mut z_levels);

    log::debug!("Rendering to svg");
    let root_svg_node = usvg::Node::new(usvg::NodeKind::Group(usvg::Group::default()));
    for z_level in z_levels {
        let render_ctx = RenderContext::new(
            render_cfg.resources,
            render_cfg.step,
            z_level,
            &layout,
            root_svg_node.clone(),
            render_cfg.default_font_name,
        );
        render_ctx.render_to_svg(&render_cfg.slide.node);
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
