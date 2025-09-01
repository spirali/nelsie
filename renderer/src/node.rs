use crate::shapes::Shape;
use crate::types::{LayoutExpr, Length, LengthOrAuto, LengthOrExpr};
use crate::{Color, NodeId};
use taffy::{
    AlignContent, AlignItems, FlexWrap, GridPlacement, Line, NonRepeatedTrackSizingFunction,
};

#[derive(Debug)]
pub enum NodeChild {
    Node(Node),
    Shape(Shape),
}

#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct ContentId(u32);

impl ContentId {
    pub fn new(image_id: u32) -> Self {
        ContentId(image_id)
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }

    pub fn bump(&mut self) -> ContentId {
        self.0 += 1;
        ContentId::new(self.0)
    }
}

#[derive(Debug)]
pub struct Node {
    pub node_id: NodeId,

    pub children: Vec<NodeChild>,

    pub show: bool,

    pub x: Option<LayoutExpr>,
    pub y: Option<LayoutExpr>,

    pub width: Option<LengthOrExpr>,
    pub height: Option<LengthOrExpr>,

    pub border_radius: f32,

    pub row: bool,

    pub reverse: bool,

    pub flex_wrap: FlexWrap,

    pub flex_grow: f32,

    pub flex_shrink: f32,

    pub align_items: Option<AlignItems>,
    pub align_self: Option<AlignItems>,
    pub justify_self: Option<AlignItems>,
    pub align_content: Option<AlignContent>,
    pub justify_content: Option<AlignContent>,

    pub column_gap: Length,
    pub row_gap: Length,

    pub grid_template_rows: Vec<NonRepeatedTrackSizingFunction>,

    pub grid_template_columns: Vec<NonRepeatedTrackSizingFunction>,

    pub grid_row: Line<GridPlacement>,
    pub grid_column: Line<GridPlacement>,

    pub p_top: Length,
    pub p_bottom: Length,
    pub p_left: Length,
    pub p_right: Length,

    pub m_top: LengthOrAuto,
    pub m_bottom: LengthOrAuto,
    pub m_left: LengthOrAuto,
    pub m_right: LengthOrAuto,

    pub bg_color: Option<Color>,

    pub z_level: i32,

    pub content: Option<ContentId>,

    pub url: Option<String>,
}

impl Node {
    pub fn child_nodes(&self) -> impl Iterator<Item = &Node> {
        self.children.iter().filter_map(|child| match child {
            NodeChild::Node(node) => Some(node),
            NodeChild::Shape(_) => None,
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
