use crate::common::Rectangle;
use crate::model::{
    InTextAnchorId, LayoutExpr, Length, LengthOrAuto, LengthOrExpr, Node, NodeContent, NodeId,
    Slide, Step,
};
use crate::render::counters::replace_counters;
use crate::render::text::get_text_layout;
use crate::render::RenderConfig;
use std::collections::{BTreeMap, HashMap};
use taffy::prelude as tf;

pub(crate) struct LayoutContext<'a> {
    config: &'a RenderConfig<'a>,
}

#[derive(Debug)]
pub(crate) struct TextLayout {
    pub lines: Vec<Rectangle>,
    pub anchor_points: HashMap<InTextAnchorId, Rectangle>,
}

#[derive(Debug)]
pub(crate) struct LayoutData {
    pub(crate) rect: Rectangle,
    pub(crate) text_layout: Option<TextLayout>,
}

#[derive(Default, Debug)]
pub(crate) struct ComputedLayout {
    node_layout: HashMap<NodeId, LayoutData>,
}

impl ComputedLayout {
    fn _layout(&self, node_id: NodeId) -> &LayoutData {
        self.node_layout(node_id)
            .expect("Node id not found, ordering not correct?")
    }

    fn _rect(&self, node_id: NodeId) -> &Rectangle {
        &self._layout(node_id).rect
    }

    pub fn eval(&self, expr: &LayoutExpr, parent_node: NodeId) -> f32 {
        match expr {
            LayoutExpr::ConstValue { value } => *value,
            LayoutExpr::X { node_id } => self._rect(*node_id).x,
            LayoutExpr::Y { node_id } => self._rect(*node_id).y,
            LayoutExpr::Width { node_id, fraction } => self._rect(*node_id).width * fraction,
            LayoutExpr::Height { node_id, fraction } => self._rect(*node_id).height * fraction,
            LayoutExpr::Sum { expressions } => {
                expressions.iter().map(|e| self.eval(e, parent_node)).sum()
            }
            LayoutExpr::ParentX { shift } => self._rect(parent_node).x + shift,
            LayoutExpr::ParentY { shift } => self._rect(parent_node).y + shift,
            LayoutExpr::ParentWidth { fraction } => self._rect(parent_node).width * fraction,
            LayoutExpr::ParentHeight { fraction } => self._rect(parent_node).height * fraction,
            LayoutExpr::LineX { node_id, line_idx } => {
                let layout = self._layout(*node_id);
                layout
                    .text_layout
                    .as_ref()
                    .and_then(|tl| tl.lines.get(*line_idx as usize).map(|line| line.x))
                    .unwrap_or(0.0)
                    + layout.rect.x
            }
            LayoutExpr::LineY { node_id, line_idx } => {
                let layout = self._layout(*node_id);
                layout
                    .text_layout
                    .as_ref()
                    .and_then(|tl| tl.lines.get(*line_idx as usize).map(|line| line.y))
                    .unwrap_or(0.0)
                    + layout.rect.y
            }
            LayoutExpr::LineWidth {
                node_id,
                line_idx,
                fraction,
            } => {
                let layout = self._layout(*node_id);
                layout
                    .text_layout
                    .as_ref()
                    .and_then(|tl| tl.lines.get(*line_idx as usize).map(|line| line.width))
                    .unwrap_or(0.0)
                    * fraction
            }
            LayoutExpr::LineHeight {
                node_id,
                line_idx,
                fraction,
            } => {
                let layout = self._layout(*node_id);
                layout
                    .text_layout
                    .as_ref()
                    .and_then(|tl| tl.lines.get(*line_idx as usize).map(|line| line.height))
                    .unwrap_or(0.0)
                    * fraction
            }
            LayoutExpr::InTextAnchorX { node_id, anchor_id } => {
                let layout = self._layout(*node_id);
                layout
                    .text_layout
                    .as_ref()
                    .and_then(|tl| tl.anchor_points.get(anchor_id).map(|a| a.x))
                    .unwrap_or(0.0)
                    + layout.rect.x
            }
            LayoutExpr::InTextAnchorY { node_id, anchor_id } => {
                let layout = self._layout(*node_id);
                layout
                    .text_layout
                    .as_ref()
                    .and_then(|tl| tl.anchor_points.get(anchor_id).map(|a| a.y))
                    .unwrap_or(0.0)
                    + layout.rect.y
            }
            LayoutExpr::InTextAnchorWidth {
                node_id,
                anchor_id,
                fraction,
            } => {
                let layout = self._layout(*node_id);
                layout
                    .text_layout
                    .as_ref()
                    .and_then(|tl| tl.anchor_points.get(anchor_id).map(|a| a.width))
                    .unwrap_or(0.0)
                    * fraction
            }
            LayoutExpr::InTextAnchorHeight {
                node_id,
                anchor_id,
                fraction,
            } => {
                let layout = self._layout(*node_id);
                layout
                    .text_layout
                    .as_ref()
                    .and_then(|tl| tl.anchor_points.get(anchor_id).map(|a| a.height))
                    .unwrap_or(0.0)
                    * fraction
            }
        }
    }

    fn set_layout(&mut self, node_id: NodeId, layout_data: LayoutData) {
        assert!(self.node_layout.insert(node_id, layout_data).is_none());
    }

    pub fn node_layout(&self, node_id: NodeId) -> Option<&LayoutData> {
        self.node_layout.get(&node_id)
    }
}

fn is_layout_managed(node: &Node, parent: Option<&Node>, step: &Step) -> bool {
    parent
        .map(|p| {
            node.main_axis_position(*p.row.at_step(step))
                .at_step(step)
                .is_none()
        })
        .unwrap_or(true)
        && node
            .width
            .at_step(step)
            .as_ref()
            .map(|v| !v.is_expr())
            .unwrap_or(true)
        && node
            .height
            .at_step(step)
            .as_ref()
            .map(|v| !v.is_expr())
            .unwrap_or(true)
}

impl From<&Length> for tf::Dimension {
    fn from(value: &Length) -> Self {
        match value {
            Length::Points { value } => tf::Dimension::Length(*value),
            Length::Fraction { value } => tf::Dimension::Percent(*value),
        }
    }
}

impl From<&Length> for tf::LengthPercentage {
    fn from(value: &Length) -> Self {
        match value {
            Length::Points { value } => tf::LengthPercentage::Length(*value),
            Length::Fraction { value } => tf::LengthPercentage::Percent(*value),
        }
    }
}

impl From<&LengthOrAuto> for tf::LengthPercentageAuto {
    fn from(value: &LengthOrAuto) -> Self {
        match value {
            LengthOrAuto::Points { value } => tf::LengthPercentageAuto::Length(*value),
            LengthOrAuto::Fraction { value } => tf::LengthPercentageAuto::Percent(*value),
            LengthOrAuto::Auto => tf::LengthPercentageAuto::Auto,
        }
    }
}

impl From<&LengthOrExpr> for tf::Dimension {
    fn from(value: &LengthOrExpr) -> Self {
        match value {
            LengthOrExpr::Points { value } => tf::Dimension::Length(*value),
            LengthOrExpr::Fraction { value } => tf::Dimension::Percent(*value),
            LengthOrExpr::Expr(_) => tf::Dimension::Auto,
        }
    }
}

fn compute_content_default_size(
    config: &RenderConfig,
    node: &Node,
    content: &NodeContent,
    step: &Step,
    text_layouts: &mut HashMap<NodeId, TextLayout>,
) -> (f32, f32) {
    match content {
        NodeContent::Text(text) => {
            let mut t = text.text_style_at_step(step);
            if text.parse_counters {
                // Here we do not "step" but "self.config.step" as we want to escape "replace_steps"
                // for counters
                replace_counters(config.counter_values, &mut t, config.slide_id, config.step);
            }
            let (width, height, text_layout) = get_text_layout(
                config.resources,
                &t,
                text.text_align,
                &text.parsed_text.at_step(step).anchors,
            );
            assert!(text_layouts.insert(node.node_id, text_layout).is_none());
            (width, height)
        }
        NodeContent::Image(image) => (image.loaded_image.width, image.loaded_image.height),
    }
}

fn gather_taffy_layout<'b>(
    step: &'b Step,
    node: &'b Node,
    parent: Option<&Node>,
    taffy: &tf::TaffyTree,
    tf_node: tf::NodeId,
    out: &mut BTreeMap<NodeId, (Option<NodeId>, &'b Node, Rectangle)>,
) {
    let step = node.replace_steps.get(step).unwrap_or(step);
    let layout_rect = taffy.layout(tf_node).unwrap();
    out.insert(
        node.node_id,
        (
            parent.map(|p| p.node_id),
            node,
            Rectangle {
                x: layout_rect.location.x,
                y: layout_rect.location.y,
                width: layout_rect.size.width,
                height: layout_rect.size.height,
            },
        ),
    );
    for (child, tf_child) in node
        .child_nodes_at_step(step)
        .zip(taffy.children(tf_node).unwrap())
    {
        gather_taffy_layout(step, child, Some(node), taffy, tf_child, out);
    }
}

impl<'a> LayoutContext<'a> {
    pub fn new(config: &'a RenderConfig<'a>) -> Self {
        LayoutContext { config }
    }

    fn compute_layout_helper(
        &self,
        step: &Step,
        taffy: &mut tf::TaffyTree,
        node: &Node,
        parent: Option<&Node>,
        text_layouts: &mut HashMap<NodeId, TextLayout>,
    ) -> tf::NodeId {
        let step = node.replace_steps.get(step).unwrap_or(step);
        let tf_children: Vec<_> = node
            .child_nodes_at_step(step)
            .map(|child| self.compute_layout_helper(step, taffy, child, Some(node), text_layouts))
            .collect();

        let w = node.width.at_step(step);
        let h = node.height.at_step(step);

        let (content_w, content_h, content_aspect_ratio) = if w.is_none() || h.is_none() {
            node.content
                .as_ref()
                .map(|content| {
                    let (content_w, content_h) = compute_content_default_size(
                        self.config,
                        node,
                        content,
                        step,
                        text_layouts,
                    );
                    if w.is_none() && h.is_none() {
                        (
                            Some(tf::Dimension::Length(content_w)),
                            Some(tf::Dimension::Length(content_h)),
                            None,
                        )
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
            .unwrap_or(tf::Dimension::Auto);
        let height = h
            .as_ref()
            .map(|v| v.into())
            .or(content_h)
            .unwrap_or(tf::Dimension::Auto);

        let flex_direction = match (node.row.at_step(step), node.reverse.at_step(step)) {
            (false, false) => tf::FlexDirection::Column,
            (true, false) => tf::FlexDirection::Row,
            (false, true) => tf::FlexDirection::ColumnReverse,
            (true, true) => tf::FlexDirection::RowReverse,
        };

        let position = if is_layout_managed(node, parent, step) {
            tf::Position::Relative
        } else {
            tf::Position::Absolute
        };

        let padding = tf::Rect {
            left: node.p_left.at_step(step).into(),
            right: node.p_right.at_step(step).into(),
            top: node.p_top.at_step(step).into(),
            bottom: node.p_bottom.at_step(step).into(),
        };

        let margin = tf::Rect {
            left: node.m_left.at_step(step).into(),
            right: node.m_right.at_step(step).into(),
            top: node.m_top.at_step(step).into(),
            bottom: node.m_bottom.at_step(step).into(),
        };

        let (gap_w, gap_h) = node.gap.at_step(step);

        /*dbg!(*node.align_items.at_step(self.step));
        dbg!(*node.align_self.at_step(self.step));
        dbg!(*node.justify_self.at_step(self.step));
        dbg!(node.align_content.at_step(self.step));
        dbg!(node.justify_content.at_step(self.step));*/

        let style = tf::Style {
            position,
            size: tf::Size { width, height },
            flex_direction,
            aspect_ratio: content_aspect_ratio,
            padding,
            margin,
            flex_wrap: *node.flex_wrap.at_step(step),
            flex_grow: *node.flex_grow.at_step(step),
            flex_shrink: *node.flex_shrink.at_step(step),

            align_items: *node.align_items.at_step(step),
            align_self: *node.align_self.at_step(step),
            justify_self: *node.justify_self.at_step(step),
            align_content: *node.align_content.at_step(step),
            justify_content: *node.justify_content.at_step(step),
            gap: tf::Size {
                width: gap_w.into(),
                height: gap_h.into(),
            },
            ..Default::default()
        };
        taffy.new_with_children(style, &tf_children).unwrap()
    }

    pub fn compute_layout(&self, slide: &Slide, step: &Step) -> ComputedLayout {
        let mut taffy = tf::TaffyTree::new();
        taffy.disable_rounding();
        let mut text_layouts = HashMap::new();
        let tf_node =
            self.compute_layout_helper(step, &mut taffy, &slide.node, None, &mut text_layouts);
        let size = tf::Size {
            width: tf::AvailableSpace::Definite(slide.width),
            height: tf::AvailableSpace::Definite(slide.height),
        };
        taffy.compute_layout(tf_node, size).unwrap();
        let mut node_entries = BTreeMap::new();
        gather_taffy_layout(step, &slide.node, None, &taffy, tf_node, &mut node_entries);
        let mut result = ComputedLayout::default();
        for (parent_id, node, rect) in node_entries.values() {
            let (parent_x, parent_y) = parent_id
                .map(|node_id| {
                    let r = &result.node_layout(node_id).unwrap().rect;
                    (r.x, r.y)
                })
                .unwrap_or((0.0, 0.0));
            let parent_id = parent_id.unwrap_or(NodeId::new(0));
            result.set_layout(
                node.node_id,
                LayoutData {
                    rect: Rectangle {
                        x: node
                            .x
                            .at_step(step)
                            .as_ref()
                            .map(|x| result.eval(x, parent_id))
                            .unwrap_or_else(|| parent_x + rect.x),
                        y: node
                            .y
                            .at_step(step)
                            .as_ref()
                            .map(|y| result.eval(y, parent_id))
                            .unwrap_or_else(|| parent_y + rect.y),
                        width: node
                            .width
                            .at_step(step)
                            .as_ref()
                            .and_then(|v| v.as_expr().map(|v| result.eval(v, parent_id)))
                            .unwrap_or(rect.width),
                        height: node
                            .height
                            .at_step(step)
                            .as_ref()
                            .and_then(|v| v.as_expr().map(|v| result.eval(v, parent_id)))
                            .unwrap_or(rect.height),
                    },
                    text_layout: text_layouts.remove(&node.node_id),
                },
            );
        }
        result
    }
}
