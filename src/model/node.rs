use super::{Color, Size, StepValue};
use crate::model::image::Image;
use crate::model::shapes::Drawing;
use crate::model::text::{FontFamily, StyledText};
use crate::model::{LayoutExpr, NodeId, Step};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use usvg::fontdb;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub(crate) enum NodeContent {
    Text(StyledText),
    Image(Image),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub(crate) enum NodeChild {
    Node(Node),
    Draw(Drawing),
}

#[derive(Debug, Deserialize)]
pub(crate) struct Node {
    pub node_id: NodeId,
    pub children: Vec<NodeChild>,

    pub show: StepValue<bool>,
    pub z_level: StepValue<i32>,

    pub x: StepValue<Option<LayoutExpr>>,
    pub y: StepValue<Option<LayoutExpr>>,

    pub width: StepValue<Option<Size>>,
    pub height: StepValue<Option<Size>>,

    pub row: StepValue<bool>,
    pub reverse: StepValue<bool>,

    pub bg_color: StepValue<Option<Color>>,
    pub content: StepValue<Option<NodeContent>>,

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

    pub fn child_nodes(&self) -> impl Iterator<Item = &Node> {
        self.children.iter().filter_map(|child| match child {
            NodeChild::Node(node) => Some(node),
            NodeChild::Draw(_) => None,
        })
    }

    pub fn collect_image_paths<'a>(&'a self, out: &mut HashSet<&'a Path>) {
        for content in self.content.values() {
            if let Some(NodeContent::Image(image)) = content {
                out.insert(image.filename.as_path());
            }
        }
        for child in self.child_nodes() {
            child.collect_image_paths(out);
        }
    }

    pub fn collect_font_families<'a>(&'a self, out: &mut HashSet<&'a FontFamily>) {
        for content in self.content.values() {
            if let Some(NodeContent::Text(text)) = content {
                for style in &text.styles {
                    out.insert(&style.font_family);
                }
            }
        }
        for child in self.child_nodes() {
            child.collect_font_families(out);
        }
    }

    pub fn collect_z_levels(&self, out: &mut BTreeSet<i32>) {
        out.extend(self.z_level.values());
        for child in self.child_nodes() {
            child.collect_z_levels(out);
        }
    }
}
