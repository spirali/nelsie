use crate::model::{LayoutExpr, Node, NodeContent, NodeId, Size, Slide, Step};
use crate::render::text::get_text_size;
use crate::render::GlobalResources;
use std::collections::{BTreeMap, HashMap};
use taffy::prelude as tf;
use taffy::style::{AvailableSpace, Dimension};
use crate::render::image::get_image_size;

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
    fn _rect(&self, node_id: NodeId) -> &Rectangle {
        self.rect(node_id)
            .expect("Node id not found, ordering not correct?")
    }

    fn eval(&self, expr: &LayoutExpr) -> f32 {
        match expr {
            LayoutExpr::ConstValue { value } => *value,
            LayoutExpr::X { node_id } => self._rect(*node_id).x,
            LayoutExpr::Y { node_id } => self._rect(*node_id).y,
            LayoutExpr::Width { node_id, fraction } => self._rect(*node_id).width * fraction,
            LayoutExpr::Height { node_id, fraction } => self._rect(*node_id).height * fraction,
            LayoutExpr::Sum { expressions } => expressions.iter().map(|e| self.eval(e)).sum(),
        }
    }

    fn set_rect(&mut self, node_id: NodeId, rect: Rectangle) {
        assert!(self.node_rects.insert(node_id, rect).is_none());
    }

    pub fn rect(&self, node_id: NodeId) -> Option<&Rectangle> {
        self.node_rects.get(&node_id)
    }
}

fn is_layout_managed(node: &Node, parent: Option<&Node>, step: Step) -> bool {
    parent
        .map(|p| {
            node.main_axis_position(*p.row.at_step(step))
                .at_step(step)
                .is_none()
        })
        .unwrap_or(true)
}

impl From<&Size> for tf::Dimension {
    fn from(value: &Size) -> Self {
        match value {
            Size::Points { value } => tf::Dimension::Points(*value),
            Size::Fraction { value } => tf::Dimension::Percent(*value),
        }
    }
}

fn compute_content_default_size(global_res: &GlobalResources, content: &NodeContent) -> (f32, f32) {
    match content {
        NodeContent::Text(text) => get_text_size(global_res.font_db(), &text),
        NodeContent::Image(image) => get_image_size(global_res, image)
    }
}

impl<'a> LayoutContext<'a> {
    pub fn new(global_res: &'a GlobalResources, step: Step) -> Self {
        LayoutContext { global_res, step }
    }

    fn compute_layout_helper(
        &self,
        taffy: &mut tf::Taffy,
        node: &Node,
        parent: Option<&Node>,
    ) -> tf::Node {
        let tf_children: Vec<_> = node
            .children
            .as_ref()
            .map(|children| {
                children
                    .iter()
                    .map(|child| self.compute_layout_helper(taffy, child, Some(node)))
                    .collect()
            })
            .unwrap_or_default();

        // let w = node.width.get(self.step);
        // let h = node.height.get(self.step);

        let w = node.width.at_step(self.step);
        let h = node.height.at_step(self.step);

        let (content_w, content_h, content_aspect_ratio) = if w.is_none() || h.is_none() {
            node.content
                .at_step(self.step)
                .as_ref()
                .map(|content| {
                    let (content_w, content_h) = compute_content_default_size(self.global_res, content);
                    if w.is_none() && h.is_none() {
                        (Some(Dimension::Points(content_w)), Some(Dimension::Points(content_h)), None)
                    } else {
                        (None, None, Some(content_w / content_h))
                    }
                })
                .unwrap_or((None, None, None))
        } else {
            (None, None, None)
        };

        let width = w
            .as_ref()
            .map(|v| v.into())
            .or(content_w)
            .unwrap_or(Dimension::Auto);
        let height = h
            .as_ref()
            .map(|v| v.into())
            .or(content_h)
            .unwrap_or(Dimension::Auto);

        let flex_direction = match (node.row.at_step(self.step), node.reverse.at_step(self.step)) {
            (false, false) => tf::FlexDirection::Column,
            (true, false) => tf::FlexDirection::Row,
            (false, true) => tf::FlexDirection::ColumnReverse,
            (true, true) => tf::FlexDirection::RowReverse,
        };

        let position = if is_layout_managed(node, parent, self.step) {
            tf::Position::Relative
        } else {
            tf::Position::Absolute
        };

        let style = tf::Style {
            position,
            size: tf::Size { width, height },
            flex_direction,
            aspect_ratio: content_aspect_ratio,
            justify_content: Some(tf::JustifyContent::Center),
            align_items: Some(tf::AlignItems::Center),
            ..Default::default()
        };
        taffy.new_with_children(style, &tf_children).unwrap()
    }

    fn gather_taffy_layout<'b>(
        &self,
        node: &'b Node,
        parent: Option<&Node>,
        taffy: &tf::Taffy,
        tf_node: tf::Node,
        out: &mut BTreeMap<NodeId, (Option<NodeId>, &'b Node, Rectangle)>,
    ) {
        let layout_rect = taffy.layout(tf_node).unwrap();
        out.insert(
            node.node_id,
            (
                parent.map(|p| p.node_id),
                &node,
                Rectangle {
                    x: layout_rect.location.x,
                    y: layout_rect.location.y,
                    width: layout_rect.size.width,
                    height: layout_rect.size.height,
                },
            ),
        );
        if let Some(children) = &node.children {
            for (child, tf_child) in children.iter().zip(taffy.children(tf_node).unwrap()) {
                self.gather_taffy_layout(child, Some(node), taffy, tf_child, out);
            }
        }
    }

    pub fn compute_layout(&self, slide: &Slide) -> ComputedLayout {
        let mut taffy = tf::Taffy::new();
        let tf_node = self.compute_layout_helper(&mut taffy, &slide.node, None);
        let size = tf::Size {
            width: AvailableSpace::Definite(slide.width),
            height: AvailableSpace::Definite(slide.height),
        };
        taffy.compute_layout(tf_node, size).unwrap();
        let mut node_entries = BTreeMap::new();
        self.gather_taffy_layout(&slide.node, None, &taffy, tf_node, &mut node_entries);
        let mut result = ComputedLayout::default();
        for (parent_id, node, rect) in node_entries.values() {
            let (parent_x, parent_y) = parent_id
                .map(|node_id| {
                    let r = result.rect(node_id).unwrap();
                    (r.x, r.y)
                })
                .unwrap_or((0.0, 0.0));
            result.set_rect(
                node.node_id,
                Rectangle {
                    x: node
                        .x
                        .at_step(self.step)
                        .as_ref()
                        .map(|x| result.eval(x))
                        .unwrap_or_else(|| parent_x + rect.x),
                    y: node
                        .y
                        .at_step(self.step)
                        .as_ref()
                        .map(|y| result.eval(y))
                        .unwrap_or_else(|| parent_y + rect.y),
                    width: rect.width,
                    height: rect.height,
                },
            );
        }
        result
    }
}
