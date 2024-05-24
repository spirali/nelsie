use super::{Color, Length, LengthOrExpr, NodeContentImage, Step, StepValue, StyleMap};
use crate::model::shapes::Drawing;
use crate::model::text::NodeContentText;
use crate::model::types::LengthOrAuto;
use crate::model::{LayoutExpr, NodeId};
use std::collections::{BTreeMap, BTreeSet, HashSet};

use by_address::ByAddress;
use std::sync::Arc;

use crate::model::image::LoadedImage;
use taffy::prelude::{AlignContent, AlignItems};
use taffy::style::FlexWrap;

#[derive(Debug)]
pub(crate) enum NodeContent {
    Text(NodeContentText),
    Image(NodeContentImage),
}

#[derive(Debug)]
pub(crate) enum NodeChild {
    Node(Node),
    Draw(Drawing),
}

#[derive(Debug)]
pub(crate) struct Node {
    pub node_id: NodeId,
    pub children: Vec<NodeChild>,

    pub replace_steps: BTreeMap<Step, Step>,

    pub active: StepValue<bool>,
    pub show: StepValue<bool>,
    pub z_level: StepValue<i32>,

    pub x: StepValue<Option<LayoutExpr>>,
    pub y: StepValue<Option<LayoutExpr>>,

    pub width: StepValue<Option<LengthOrExpr>>,
    pub height: StepValue<Option<LengthOrExpr>>,

    pub border_radius: StepValue<f32>,

    pub row: StepValue<bool>,
    pub reverse: StepValue<bool>,

    pub flex_wrap: StepValue<FlexWrap>,
    pub flex_grow: StepValue<f32>,
    pub flex_shrink: StepValue<f32>,

    pub align_items: StepValue<Option<AlignItems>>,
    pub align_self: StepValue<Option<AlignItems>>,
    pub justify_self: StepValue<Option<AlignItems>>,
    pub align_content: StepValue<Option<AlignContent>>,
    pub justify_content: StepValue<Option<AlignContent>>,
    pub gap: StepValue<(Length, Length)>,

    pub p_top: StepValue<Length>,
    pub p_bottom: StepValue<Length>,
    pub p_left: StepValue<Length>,
    pub p_right: StepValue<Length>,

    pub m_top: StepValue<LengthOrAuto>,
    pub m_bottom: StepValue<LengthOrAuto>,
    pub m_left: StepValue<LengthOrAuto>,
    pub m_right: StepValue<LengthOrAuto>,

    pub bg_color: StepValue<Option<Color>>,
    pub content: Option<NodeContent>,

    pub url: StepValue<Option<String>>,

    pub styles: Arc<StyleMap>,

    pub name: String,
    pub debug_layout: Option<Color>,
}

impl Node {
    pub fn main_axis_position(&self, has_row_parent: bool) -> &StepValue<Option<LayoutExpr>> {
        if has_row_parent {
            &self.x
        } else {
            &self.y
        }
    }

    pub fn child_nodes_at_step<'a>(
        &'a self,
        step: &'a Step,
    ) -> impl Iterator<Item = &'a Node> + 'a {
        self.children.iter().filter_map(move |child| match child {
            NodeChild::Node(node) if *node.active.at_step(step) => Some(node),
            _ => None,
        })
    }

    pub fn child_nodes(&self) -> impl Iterator<Item = &Node> {
        self.children.iter().filter_map(|child| match child {
            NodeChild::Node(node) => Some(node),
            NodeChild::Draw(_) => None,
        })
    }

    pub fn child_node_mut(&mut self, node_idx: usize) -> Option<&mut Node> {
        self.children
            .get_mut(node_idx)
            .and_then(|child| match child {
                NodeChild::Node(node) => Some(node),
                NodeChild::Draw(_) => None,
            })
    }

    pub fn collect_z_levels(&self, out: &mut BTreeSet<i32>) {
        out.extend(self.z_level.values());
        for child in self.child_nodes() {
            child.collect_z_levels(out);
        }
    }

    pub fn collect_images(&self, out: &mut HashSet<ByAddress<Arc<LoadedImage>>>) {
        if let Some(NodeContent::Image(image)) = &self.content {
            out.insert(ByAddress::from(image.loaded_image.clone()));
        };
        for child in self.child_nodes() {
            child.collect_images(out);
        }
    }
}
