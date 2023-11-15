use std::collections::{BTreeMap, HashMap};
use crate::model::{Node, NodeId, PosAndSizeExpr, Size, Slide, Step};
use crate::render::text::get_text_size;
use crate::render::GlobalResources;
use taffy::{prelude as tf};
use taffy::style::AvailableSpace;

pub(crate) struct LayoutContext<'a> {
    global_res: &'a GlobalResources,
    step: Step,
}

#[derive(Debug)]
pub(crate) struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Default, Debug)]
pub(crate) struct ComputedLayout {
    node_rects: HashMap<NodeId, Rectangle>,
}

impl ComputedLayout {
    fn eval(&self, current_id: NodeId, expr: &PosAndSizeExpr) -> f32 {
        match expr {
            PosAndSizeExpr::Const { value } => *value,
            PosAndSizeExpr::X { .. } => { todo!() }
            PosAndSizeExpr::Y { .. } => { todo!() }
            PosAndSizeExpr::Width { node_id, fraction } => {
                self.node_rects[&node_id].width * fraction
            }
            PosAndSizeExpr::Height { node_id, fraction } => {
                self.node_rects[&node_id].height * fraction
            }
            PosAndSizeExpr::Sum { expressions } => { expressions.iter().map(|e| self.eval(current_id, e)).sum() }
        }
    }

    fn set_rect(&mut self, node_id: NodeId, rect: Rectangle) {
        assert!(self.node_rects.insert(node_id, rect).is_none());
    }

    pub fn rect(&self, node_id: NodeId) -> Option<&Rectangle> {
        self.node_rects.get(&node_id)
    }
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

        let (width, height) = if let Some(text) = &node.text.at_step(self.step) {
            let (width, height) = get_text_size(self.global_res.font_db(), &text);
            (tf::Dimension::Points(width), tf::Dimension::Points(height))
        } else {
            (
                node.width.at_step(self.step).into(),
                node.height.at_step(self.step).into(),
            )
        };

        let flex_direction = match (node.row.at_step(self.step), node.reverse.at_step(self.step)) {
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

    fn gather_taffy_layout<'b>(&self, node: &'b Node, parent: Option<&Node>, taffy: &tf::Taffy, tf_node: tf::Node, out: &mut BTreeMap<NodeId, (Option<NodeId>, &'b Node, Rectangle)>) {
        let layout_rect = taffy.layout(tf_node).unwrap();
        out.insert(node.node_id, (
            parent.map(|p| p.node_id),
            &node, Rectangle {
                x: layout_rect.location.x,
                y: layout_rect.location.y,
                width: layout_rect.size.width,
                height: layout_rect.size.height,
            }));
        if let Some(children) = &node.children {
            for (child, tf_child) in children.iter().zip(taffy.children(tf_node).unwrap()) {
                self.gather_taffy_layout(child, Some(node), taffy, tf_child, out);
            }
        }
    }

    pub fn compute_layout(&self, slide: &Slide) -> ComputedLayout {
        let mut taffy = tf::Taffy::new();
        let tf_node = self.compute_layout_helper(&mut taffy, &slide.node);
        let size = tf::Size {
            width: AvailableSpace::Definite(slide.width),
            height: AvailableSpace::Definite(slide.height),
        };
        taffy.compute_layout(tf_node, size).unwrap();
        let mut node_entries = BTreeMap::new();
        self.gather_taffy_layout(&slide.node, None, &taffy, tf_node, &mut node_entries);
        let mut result = ComputedLayout::default();
        for (parent_id, node, rect) in node_entries.values() {
            let (parent_x, parent_y) = parent_id.map(|node_id| {
                let r = result.rect(node_id).unwrap();
                (r.x, r.y)
            }).unwrap_or((0.0, 0.0));
            result.set_rect(node.node_id, Rectangle {
                x: parent_x + rect.x,
                y: parent_y + rect.y,
                width: rect.width,
                height: rect.height,
            });
        }
        result
    }
}
