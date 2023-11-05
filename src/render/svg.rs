use super::text::{get_text_size, render_text};
use crate::model::{Color, Node, Step};
use crate::render::core::RenderConfig;
use crate::render::layout::LayoutContext;
use resvg::tiny_skia;
use std::rc::Rc;
use taffy::style::{Dimension, FlexDirection, JustifyContent, Style};
use taffy::style_helpers::TaffyMaxContent;
use taffy::{prelude as tf, Taffy};
use usvg;
use usvg::{
    CharacterPosition, Fill, NonZeroPositiveF32, Text, TextAnchor, TextChunk, TextFlow,
    TextRendering, WritingMode,
};
use usvg_tree::{
    AlignmentBaseline, DominantBaseline, Font, FontStretch, FontStyle, LengthAdjust, PaintOrder,
    TextDecoration, TextSpan, Visibility,
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

    fn render_helper(&self, node: &Node, tf_node: tf::Node) {
        let layout = self.taffy.layout(tf_node).unwrap();
        if let Some(color) = &node.bg_color.get(self.step) {
            let mut path = usvg::Path::new(Rc::new(tiny_skia::PathBuilder::from_rect(
                tiny_skia::Rect::from_xywh(
                    layout.location.x,
                    layout.location.y,
                    layout.size.width,
                    layout.size.height,
                )
                .unwrap(),
            )));
            path.fill = Some(Fill {
                paint: usvg::Paint::Color(color.into()),
                ..Default::default()
            });
            self.svg_node
                .append(usvg::Node::new(usvg::NodeKind::Path(path)));
        }

        if let Some(text) = &node.text {
            self.svg_node.append(render_text(
                &text,
                layout.location.x,
                layout.location.y + layout.size.height - 7.0,
            ));
        }

        for (n, tf_n) in node
            .children
            .iter()
            .zip(self.taffy.children(tf_node).unwrap())
        {
            self.render_helper(n, tf_n);
        }
    }

    pub(crate) fn render_to_svg(self, node: &Node, tf_node: tf::Node) -> usvg::Node {
        self.render_helper(node, tf_node);
        self.svg_node
    }
}

pub(crate) fn render_to_svg_tree(render_cfg: &RenderConfig) -> usvg_tree::Tree {
    println!("Creating layout");
    let layout_builder = LayoutContext::new(render_cfg.step);
    let (taffy, tf_node) = layout_builder.compute_layout(render_cfg.slide);

    println!("Rendering to svg");
    let render_ctx = RenderContext::new(render_cfg.step, 0, &taffy);
    let root_svg_node = render_ctx.render_to_svg(&render_cfg.slide.node, tf_node);

    let size = usvg::Size::from_wh(render_cfg.slide.width, render_cfg.slide.height).unwrap();
    let tree = usvg_tree::Tree {
        size,
        view_box: usvg::ViewBox {
            rect: size.to_non_zero_rect(0.0, 0.0),
            aspect: usvg::AspectRatio::default(),
        },
        root: root_svg_node,
    };
    tree
}

#[cfg(test)]
mod tests {
    use super::super::text::get_text_size;
    use super::render_to_svg_tree;
    use crate::common::Size;
    use crate::common::{Node, StepValue};
    use usvg::TreeTextToPath;
    use usvg::{fontdb, Color, TreeParsing, TreeWriting, XmlOptions};

    #[test]
    fn test_render() {
        let node = Node {
            width: StepValue::Const(Size::Points(800.0)),
            height: StepValue::Const(Size::Points(600.0)),
            bg_color: None,
            text: None,
            children: vec![
                Node {
                    width: StepValue::Const(Size::Points(100.0)),
                    height: StepValue::Const(Size::Points(100.0)),
                    children: vec![],
                    bg_color: Some(Color::new_rgb(255, 0, 0)),
                    text: Some("Ahoj světe!!!".to_string()),
                },
                Node {
                    width: StepValue::Const(Size::Points(300.0)),
                    height: StepValue::Const(Size::Points(200.0)),
                    children: vec![],
                    text: None,
                    bg_color: Some(Color::new_rgb(0, 0, 255)),
                },
            ],
        };
        let mut tree = render_to_svg_tree(&node);
        println!("{}", tree.to_string(&XmlOptions::default()));

        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();
        tree.convert_text(&fontdb);

        println!("{}", tree.to_string(&XmlOptions::default()));

        let pdf = svg2pdf::convert_tree(&tree, svg2pdf::Options::default());
        std::fs::write("/tmp/x.pdf", pdf).unwrap();
    }

    #[test]
    fn test_xxx() {
        let svg_data = std::fs::read("/home/spirali/projects/nelsie/test3.svg").unwrap();
        let mut tree = usvg::Tree::from_data(&svg_data, &Default::default()).unwrap();
        println!("{:?}", tree.root.first_child().unwrap());
        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();
        tree.convert_text(&fontdb);
        println!("{}", tree.to_string(&Default::default()));
    }

    #[test]
    fn test_text_size() {
        println!("{:?}", get_text_size("Ahoj"));
    }
}
