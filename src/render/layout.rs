use crate::model::{Node, Size, Slide, Step};
use crate::render::text::get_text_size;
use crate::render::GlobalResources;
use taffy::prelude as tf;

use taffy::style::AvailableSpace;

pub(crate) struct LayoutContext<'a> {
    global_res: &'a GlobalResources,
    step: Step,
}

impl From<&Size> for tf::Dimension {
    fn from(value: &Size) -> Self {
        match value {
            Size::Points(v) => tf::Dimension::Points(*v),
            Size::Percent(v) => tf::Dimension::Percent(*v / 100.0),
            Size::Auto => tf::Dimension::Auto,
        }
    }
}

impl<'a> LayoutContext<'a> {
    pub fn new(global_res: &'a GlobalResources, step: Step) -> Self {
        LayoutContext { global_res, step }
    }

    fn compute_layout_helper(&self, taffy: &mut tf::Taffy, node: &Node) -> tf::Node {
        let tf_children: Vec<_> = node
            .children
            .as_ref()
            .map(|children| {
                children
                    .iter()
                    .map(|n| self.compute_layout_helper(taffy, n))
                    .collect()
            })
            .unwrap_or_default();

        // let w = node.width.get(self.step);
        // let h = node.height.get(self.step);

        let (width, height) = if let Some(text) = &node.text.get(self.step) {
            let (width, height) = get_text_size(self.global_res.font_db(), text);
            (tf::Dimension::Points(width), tf::Dimension::Points(height))
        } else {
            (
                node.width.get(self.step).into(),
                node.height.get(self.step).into(),
            )
        };

        let flex_direction = match (node.row.get(self.step), node.reverse.get(self.step)) {
            (false, false) => tf::FlexDirection::Column,
            (true, false) => tf::FlexDirection::Row,
            (false, true) => tf::FlexDirection::ColumnReverse,
            (true, true) => tf::FlexDirection::RowReverse,
        };

        let style = tf::Style {
            size: tf::Size { width, height },
            flex_direction,
            justify_content: Some(tf::JustifyContent::Center),
            align_items: Some(tf::AlignItems::Center),
            ..Default::default()
        };
        taffy.new_with_children(style, &tf_children).unwrap()
    }

    pub fn compute_layout(&self, slide: &Slide) -> (tf::Taffy, tf::Node) {
        let mut taffy = tf::Taffy::new();
        let tf_node = self.compute_layout_helper(&mut taffy, &slide.node);
        let size = tf::Size {
            width: AvailableSpace::Definite(slide.width),
            height: AvailableSpace::Definite(slide.height),
        };
        taffy.compute_layout(tf_node, size).unwrap();
        (taffy, tf_node)
    }
}
