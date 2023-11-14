use std::collections::HashMap;
use crate::model::{Node, NodeId, PosAndSizeExpr, Size, Slide, Step};
use crate::render::text::get_text_size;
use crate::render::GlobalResources;
use taffy::prelude as tf;

use taffy::style::AvailableSpace;

pub(crate) struct LayoutContext<'a> {
    global_res: &'a GlobalResources,
    step: Step,
}

#[derive(Debug)]
struct ExprNodeRect {
    x: PosAndSizeExpr,
    y: PosAndSizeExpr,
    width: PosAndSizeExpr,
    height: PosAndSizeExpr,
}

#[derive(Default, Debug)]
pub(crate) struct ComputedLayout {
    node_rects: HashMap<NodeId, ExprNodeRect>,
}

impl ComputedLayout {
    fn eval(&self, current_id: NodeId, expr: &PosAndSizeExpr) -> f32 {
        match expr {
            PosAndSizeExpr::Const { value } => *value,
            PosAndSizeExpr::X { .. } => { todo!() }
            PosAndSizeExpr::Y { .. } => { todo!() }
            PosAndSizeExpr::Width { node_id, fraction } => {
                assert!(*node_id < current_id);
                self.eval(*node_id, &self.node_rects[&node_id].width) * fraction
            }
            PosAndSizeExpr::Height { node_id, fraction } => {
                assert!(*node_id < current_id);
                self.eval(*node_id, &self.node_rects[&node_id].height) * fraction
            }
            PosAndSizeExpr::Sum { expressions } => { expressions.iter().map(|e| self.eval(current_id, e)).sum() }
        }
    }

    fn set_rect(&mut self, node_id: NodeId, rect: ExprNodeRect) {
        assert!(self.node_rects.insert(node_id, rect).is_none());
    }

    fn rect(&self, node_id: NodeId) -> Option<&ExprNodeRect> {
        self.node_rects.get(&node_id)
    }

    pub fn xywh(&self, node_id: NodeId) -> (f32, f32, f32, f32) {
        let rect = &self.node_rects[&node_id];
        (self.eval(node_id, &rect.x), self.eval(node_id, &rect.y), self.eval(node_id, &rect.width), self.eval(node_id, &rect.height))
    }

    pub fn xy(&self, node_id: NodeId) -> (f32, f32) {
        let rect = &self.node_rects[&node_id];
        (self.eval(node_id, &rect.x), self.eval(node_id, &rect.y))
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

    pub fn compose_final_layer_helper(&self, node: &Node, parent: Option<&Node>, taffy: &tf::Taffy, tf_node: tf::Node, out: &mut ComputedLayout) {
        let layout_rect = taffy.layout(tf_node).unwrap();
        let (p_x, p_y) = parent.map(|p| {
            let r = out.rect(p.node_id).unwrap();
            (r.x.clone(), r.y.clone())
        }).unwrap_or_else(|| (PosAndSizeExpr::new_const(0.0), PosAndSizeExpr::new_const(0.0)));
        let x = PosAndSizeExpr::new_sum(&p_x, &PosAndSizeExpr::new_const(layout_rect.location.x));
        let y = PosAndSizeExpr::new_sum(&p_y, &PosAndSizeExpr::new_const(layout_rect.location.y));
        let width = PosAndSizeExpr::new_const(layout_rect.size.width);
        let height = PosAndSizeExpr::new_const(layout_rect.size.height);
        out.set_rect(node.node_id, ExprNodeRect {
            x,
            y,
            width,
            height,
        });
        if let Some(children) = &node.children {
            for (child, tf_child) in children.iter().zip(taffy.children(tf_node).unwrap()) {
                self.compose_final_layer_helper(child, Some(node), taffy, tf_child, out);
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
        let mut result = ComputedLayout::default();
        self.compose_final_layer_helper(&slide.node, None, &taffy, tf_node, &mut result);
        result
    }
}
