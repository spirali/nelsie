use super::text::{render_text};
use crate::model::{Color, Node, Step};
use crate::render::core::RenderConfig;
use crate::render::layout::LayoutContext;

use resvg::tiny_skia;
use std::rc::Rc;


use taffy::{prelude as tf, Taffy};

use usvg::{
    Fill,
};


pub(crate) struct RenderContext<'a> {
    step: Step,
    z_level: i32,
    taffy: &'a Taffy,
    svg_node: usvg::Node,
}

impl From<&Color> for usvg::Color {
    fn from(value: &Color) -> Self {
        let c: svgtypes::Color = value.into();
        usvg::Color::new_rgb(c.red, c.green, c.blue)
    }
}

impl<'a> RenderContext<'a> {
    pub fn new(step: Step, z_level: i32, taffy: &'a Taffy) -> Self {
        RenderContext {
            step,
            z_level,
            taffy,
            svg_node: usvg::Node::new(usvg::NodeKind::Group(usvg::Group::default())),
        }
    }

    fn render_helper(&self, node: &Node, parent_x: f32, parent_y: f32, tf_node: tf::Node) {
        if !node.show.at_step(self.step) {
            return;
        }
        let layout = self.taffy.layout(tf_node).unwrap();
        let x = layout.location.x + parent_x;
        let y = layout.location.y + parent_y;

        if let Some(color) = &node.bg_color.at_step(self.step) {
            let mut path = usvg::Path::new(Rc::new(tiny_skia::PathBuilder::from_rect(
                tiny_skia::Rect::from_xywh(x, y, layout.size.width, layout.size.height).unwrap(),
            )));
            path.fill = Some(Fill {
                paint: usvg::Paint::Color(color.into()),
                ..Default::default()
            });
            self.svg_node
                .append(usvg::Node::new(usvg::NodeKind::Path(path)));
        }

        if let Some(text) = &node.text.at_step(self.step) {
            self.svg_node.append(render_text(&text, x, y));
        }

        if let Some(children) = &node.children {
            for (n, tf_n) in children.iter().zip(self.taffy.children(tf_node).unwrap()) {
                self.render_helper(n, x, y, tf_n);
            }
        }
    }

    pub(crate) fn render_to_svg(self, node: &Node, tf_node: tf::Node) -> usvg::Node {
        self.render_helper(node, 0.0, 0.0, tf_node);
        self.svg_node
    }
}

pub(crate) fn render_to_svg_tree(render_cfg: &RenderConfig) -> usvg_tree::Tree {
    log::debug!("Creating layout");
    let layout_builder = LayoutContext::new(render_cfg.global_res, render_cfg.step);
    let (taffy, tf_node) = layout_builder.compute_layout(render_cfg.slide);

    log::debug!("Rendering to svg");
    let render_ctx = RenderContext::new(render_cfg.step, 0, &taffy);
    let root_svg_node = render_ctx.render_to_svg(&render_cfg.slide.node, tf_node);

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
