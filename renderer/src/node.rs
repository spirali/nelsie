use crate::types::{LayoutExpr, Length, LengthOrAuto, LengthOrExpr};
use crate::{Color, NodeId};
use bon::Builder;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use taffy::{
    AlignContent, AlignItems, FlexWrap, GridPlacement, Line, NonRepeatedTrackSizingFunction,
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum NodeChild {
    Node(Node),
    //    Draw(Drawing),
}

#[derive(Builder, Debug)]
pub struct Node {
    #[builder(start_fn)]
    pub node_id: NodeId,

    #[builder(skip)]
    pub children: Vec<NodeChild>,

    #[builder(default = true)]
    pub show: bool,

    pub x: Option<LayoutExpr>,
    pub y: Option<LayoutExpr>,

    pub width: Option<LengthOrExpr>,
    pub height: Option<LengthOrExpr>,

    #[builder(default)]
    pub border_radius: f32,

    #[builder(default)]
    pub row: bool,

    #[builder(default)]
    pub reverse: bool,

    #[builder(default)]
    pub flex_wrap: FlexWrap,

    #[builder(default)]
    pub flex_grow: f32,

    #[builder(default)]
    pub flex_shrink: f32,

    pub align_items: Option<AlignItems>,
    pub align_self: Option<AlignItems>,
    pub justify_self: Option<AlignItems>,
    pub align_content: Option<AlignContent>,
    pub justify_content: Option<AlignContent>,

    #[builder(default)]
    pub column_gap: Length,

    #[builder(default)]
    pub row_gap: Length,

    #[builder(default)]
    pub grid_template_rows: Vec<NonRepeatedTrackSizingFunction>,

    #[builder(default)]
    pub grid_template_columns: Vec<NonRepeatedTrackSizingFunction>,

    #[builder(default)]
    pub grid_row: Line<GridPlacement>,

    #[builder(default)]
    pub grid_column: Line<GridPlacement>,

    #[builder(default)]
    pub p_top: Length,

    #[builder(default)]
    pub p_bottom: Length,

    #[builder(default)]
    pub p_left: Length,

    #[builder(default)]
    pub p_right: Length,

    #[builder(default)]
    pub m_top: LengthOrAuto,

    #[builder(default)]
    pub m_bottom: LengthOrAuto,

    #[builder(default)]
    pub m_left: LengthOrAuto,

    #[builder(default)]
    pub m_right: LengthOrAuto,

    pub bg_color: Option<Color>,

    #[builder(default)]
    pub z_level: i32,

    //pub content: Option<NodeContent>,
    pub url: Option<String>,
}

impl Node {
    pub fn child_nodes(&self) -> impl Iterator<Item = &Node> {
        self.children.iter().filter_map(|child| match child {
            NodeChild::Node(node) => Some(node),
            //NodeChild::Draw(_) => None,
        })
    }

    pub fn main_axis_position(&self, has_row_parent: bool) -> Option<&LayoutExpr> {
        if has_row_parent {
            self.x.as_ref()
        } else {
            self.y.as_ref()
        }
    }

    pub fn add_child_node(&mut self, node: Node) {
        self.children.push(NodeChild::Node(node));
    }

    // fn collect_z_levels(&self, out: &mut BTreeSet<i32>) {
    //     out.insert(self.z_level);
    //     for child in self.child_nodes() {
    //         child.collect_z_levels(out);
    //     }
    // }
}
