use crate::node::Node;
use crate::render::context::RenderContext;
use crate::render::text::RenderedText;
use crate::types::{LayoutExpr, Length, LengthOrAuto, LengthOrExpr};
use crate::{NodeId, Page, Rectangle};
use itertools::Itertools;
use std::collections::HashMap;
use std::sync::Arc;
use taffy::{AlignItems, Display, JustifyContent, prelude as tf};

#[derive(Debug)]
pub(crate) struct LayoutData {
    pub(crate) rect: Rectangle,
    pub(crate) text: Option<Arc<RenderedText>>,
}

#[derive(Debug)]
pub(crate) struct ComputedLayout {
    node_layout: HashMap<NodeId, LayoutData>,
}

impl ComputedLayout {
    pub fn new(capacity: usize) -> Self {
        ComputedLayout {
            node_layout: HashMap::with_capacity(capacity),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&NodeId, &LayoutData)> {
        self.node_layout.iter()
    }

    fn _layout(&self, node_id: NodeId) -> &LayoutData {
        self.node_layout(node_id)
            .unwrap_or_else(|| panic!("Node {node_id:?} not found, ordering not correct?"))
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
            LayoutExpr::Add { expressions } => {
                self.eval(&expressions.0, parent_node) + self.eval(&expressions.1, parent_node)
            }
            LayoutExpr::Sub { expressions } => {
                self.eval(&expressions.0, parent_node) - self.eval(&expressions.1, parent_node)
            }
            LayoutExpr::Mul { expressions } => {
                self.eval(&expressions.0, parent_node) * self.eval(&expressions.1, parent_node)
            }
            LayoutExpr::Max { expressions } => expressions
                .iter()
                .map(|e| self.eval(e, parent_node))
                .fold(0.0, |a, b| a.max(b)),
            LayoutExpr::ParentX { shift } => self._rect(parent_node).x + shift,
            LayoutExpr::ParentY { shift } => self._rect(parent_node).y + shift,
            LayoutExpr::ParentWidth { fraction } => self._rect(parent_node).width * fraction,
            LayoutExpr::ParentHeight { fraction } => self._rect(parent_node).height * fraction,
            LayoutExpr::LineX { node_id, line_idx } => {
                let layout = self._layout(*node_id);
                layout
                    .text
                    .as_ref()
                    .and_then(|tl| tl.line_layouts().get(*line_idx as usize).map(|line| line.x))
                    .unwrap_or(0.0)
                    + layout.rect.x
            }
            LayoutExpr::LineY { node_id, line_idx } => {
                let layout = self._layout(*node_id);
                layout
                    .text
                    .as_ref()
                    .and_then(|tl| tl.line_layouts().get(*line_idx as usize).map(|line| line.y))
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
                    .text
                    .as_ref()
                    .and_then(|tl| {
                        tl.line_layouts()
                            .get(*line_idx as usize)
                            .map(|line| line.width)
                    })
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
                    .text
                    .as_ref()
                    .and_then(|tl| {
                        tl.line_layouts()
                            .get(*line_idx as usize)
                            .map(|line| line.height)
                    })
                    .unwrap_or(0.0)
                    * fraction
            }
            LayoutExpr::InlineX {
                node_id,
                inline_id: anchor_id,
            } => {
                let layout = self._layout(*node_id);
                layout
                    .text
                    .as_ref()
                    .and_then(|tl| tl.intext_rects().get(anchor_id).map(|a| a.x))
                    .unwrap_or(0.0)
                    + layout.rect.x
            }
            LayoutExpr::InlineY {
                node_id,
                inline_id: anchor_id,
            } => {
                let layout = self._layout(*node_id);
                layout
                    .text
                    .as_ref()
                    .and_then(|tl| tl.intext_rects().get(anchor_id).map(|a| a.y))
                    .unwrap_or(0.0)
                    + layout.rect.y
            }
            LayoutExpr::InlineWidth {
                node_id,
                inline_id: anchor_id,
                fraction,
            } => {
                let layout = self._layout(*node_id);
                layout
                    .text
                    .as_ref()
                    .and_then(|tl| tl.intext_rects().get(anchor_id).map(|a| a.width))
                    .unwrap_or(0.0)
                    * fraction
            }
            LayoutExpr::InlineHeight {
                node_id,
                inline_id: anchor_id,
                fraction,
            } => {
                let layout = self._layout(*node_id);
                layout
                    .text
                    .as_ref()
                    .and_then(|tl| tl.intext_rects().get(anchor_id).map(|a| a.height))
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

fn is_layout_managed(node: &Node, parent: Option<&Node>) -> bool {
    parent
        .map(|p| node.main_axis_position(p.row).is_none())
        .unwrap_or(true)
        && node.width.as_ref().map(|v| !v.is_expr()).unwrap_or(true)
        && node.height.as_ref().map(|v| !v.is_expr()).unwrap_or(true)
}

impl From<Length> for tf::Dimension {
    fn from(value: Length) -> Self {
        match value {
            Length::Points { value } => tf::Dimension::Length(value),
            Length::Fraction { value } => tf::Dimension::Percent(value),
        }
    }
}

impl From<Length> for tf::LengthPercentage {
    fn from(value: Length) -> Self {
        match value {
            Length::Points { value } => tf::LengthPercentage::Length(value),
            Length::Fraction { value } => tf::LengthPercentage::Percent(value),
        }
    }
}

impl From<LengthOrAuto> for tf::LengthPercentageAuto {
    fn from(value: LengthOrAuto) -> Self {
        match value {
            LengthOrAuto::Length(Length::Points { value }) => {
                tf::LengthPercentageAuto::Length(value)
            }
            LengthOrAuto::Length(Length::Fraction { value }) => {
                tf::LengthPercentageAuto::Percent(value)
            }
            LengthOrAuto::Auto => tf::LengthPercentageAuto::Auto,
        }
    }
}

impl From<&LengthOrExpr> for tf::Dimension {
    fn from(value: &LengthOrExpr) -> Self {
        match value {
            LengthOrExpr::Length(Length::Points { value }) => tf::Dimension::Length(*value),
            LengthOrExpr::Length(Length::Fraction { value }) => tf::Dimension::Percent(*value),
            LengthOrExpr::Expr(_) => tf::Dimension::Auto,
        }
    }
}

fn gather_taffy_layout<'b>(
    node: &'b Node,
    parent: Option<&Node>,
    taffy: &tf::TaffyTree,
    tf_node: tf::NodeId,
    out: &mut HashMap<NodeId, (Option<NodeId>, &'b Node, Rectangle)>,
) {
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
    for (child, tf_child) in node.child_nodes().zip(taffy.children(tf_node).unwrap()) {
        gather_taffy_layout(child, Some(node), taffy, tf_child, out);
    }
}

fn compute_layout_helper(
    render_ctx: &mut RenderContext,
    taffy: &mut tf::TaffyTree,
    node: &Node,
    parent: Option<&Node>,
    node_id_order: &mut Vec<NodeId>,
) -> tf::NodeId {
    node_id_order.push(node.node_id);
    let tf_children: Vec<_> = node
        .child_nodes()
        .map(|child| compute_layout_helper(render_ctx, taffy, child, Some(node), node_id_order))
        .collect();

    let w = node.width.as_ref();
    let h = node.height.as_ref();

    let (content_w, content_h, content_aspect_ratio) = if w.is_none() || h.is_none() {
        if let Some(content) = node.content.as_ref() {
            let (content_w, content_h) = render_ctx.content_map.get(content).unwrap().size();
            if w.is_none() && h.is_none() {
                (
                    Some(tf::Dimension::Length(content_w)),
                    Some(tf::Dimension::Length(content_h)),
                    None,
                )
            } else {
                (None, None, Some(content_w / content_h))
            }
        } else {
            (None, None, None)
        }
    } else {
        (None, None, None)
    };

    let width = w
        .as_ref()
        .map(|v| (*v).into())
        .or(content_w)
        .unwrap_or(tf::Dimension::Auto);
    let height = h
        .as_ref()
        .map(|v| (*v).into())
        .or(content_h)
        .unwrap_or(tf::Dimension::Auto);

    let flex_direction = match (node.row, node.reverse) {
        (false, false) => tf::FlexDirection::Column,
        (true, false) => tf::FlexDirection::Row,
        (false, true) => tf::FlexDirection::ColumnReverse,
        (true, true) => tf::FlexDirection::RowReverse,
    };

    let position = if is_layout_managed(node, parent) {
        tf::Position::Relative
    } else {
        tf::Position::Absolute
    };

    let padding = tf::Rect {
        left: node.p_left.into(),
        right: node.p_right.into(),
        top: node.p_top.into(),
        bottom: node.p_bottom.into(),
    };

    let margin = tf::Rect {
        left: node.m_left.into(),
        right: node.m_right.into(),
        top: node.m_top.into(),
        bottom: node.m_bottom.into(),
    };

    /*dbg!(*node.align_items.at_step(self.step));
    dbg!(*node.align_self.at_step(self.step));
    dbg!(*node.justify_self.at_step(self.step));
    dbg!(node.align_content.at_step(self.step));
    dbg!(node.justify_content.at_step(self.step));*/

    let grid_template_rows = node
        .grid_template_rows
        .iter()
        .map(|x| tf::TrackSizingFunction::Single(*x))
        .collect_vec();
    let grid_template_columns = node
        .grid_template_columns
        .iter()
        .map(|x| tf::TrackSizingFunction::Single(*x))
        .collect_vec();
    let is_grid = !grid_template_rows.is_empty() || !grid_template_columns.is_empty();

    let style = tf::Style {
        display: if is_grid {
            Display::Grid
        } else {
            Display::Flex
        },
        position,
        size: tf::Size { width, height },
        flex_direction,
        aspect_ratio: content_aspect_ratio,
        padding,
        margin,
        flex_wrap: node.flex_wrap,
        flex_grow: node.flex_grow,
        flex_shrink: node.flex_shrink,
        align_items: node.align_items.or({
            if is_grid {
                None
            } else {
                Some(AlignItems::Center)
            }
        }),
        align_self: node.align_self,
        justify_self: node.justify_self,
        align_content: node.align_content,
        justify_content: node.justify_content.or({
            if is_grid {
                None
            } else {
                Some(JustifyContent::Center)
            }
        }),
        gap: tf::Size {
            width: node.column_gap.into(),
            height: node.row_gap.into(),
        },
        grid_template_rows,
        grid_template_columns,
        grid_row: node.grid_row,
        grid_column: node.grid_column,
        ..Default::default()
    };
    taffy.new_with_children(style, &tf_children).unwrap()
}

pub fn compute_page_layout(render_ctx: &mut RenderContext, page: &Page) -> ComputedLayout {
    let mut taffy = tf::TaffyTree::new();
    taffy.disable_rounding();
    let mut node_id_order = Vec::with_capacity(16);
    let tf_node =
        compute_layout_helper(render_ctx, &mut taffy, &page.node, None, &mut node_id_order);
    let size = tf::Size {
        width: tf::AvailableSpace::Definite(page.width),
        height: tf::AvailableSpace::Definite(page.height),
    };
    taffy.compute_layout(tf_node, size).unwrap();
    let mut node_entries = HashMap::with_capacity(node_id_order.len());
    gather_taffy_layout(&page.node, None, &taffy, tf_node, &mut node_entries);
    let mut result = ComputedLayout::new(node_id_order.len());
    for node_id in node_id_order {
        let (parent_id, node, rect) = node_entries.get(&node_id).unwrap();
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
                        .as_ref()
                        .map(|x| result.eval(x, parent_id))
                        .unwrap_or_else(|| parent_x + rect.x),
                    y: node
                        .y
                        .as_ref()
                        .map(|y| result.eval(y, parent_id))
                        .unwrap_or_else(|| parent_y + rect.y),
                    width: node
                        .width
                        .as_ref()
                        .and_then(|v| v.as_expr().map(|v| result.eval(v, parent_id)))
                        .unwrap_or(rect.width),
                    height: node
                        .height
                        .as_ref()
                        .and_then(|v| v.as_expr().map(|v| result.eval(v, parent_id)))
                        .unwrap_or(rect.height),
                },
                text: node.content.and_then(|content_id| {
                    render_ctx
                        .content_map
                        .get(&content_id)
                        .and_then(|c| c.as_text().cloned())
                }),
            },
        );
    }
    result
}
