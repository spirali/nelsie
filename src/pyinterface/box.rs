use std::path::PathBuf;
use log::__private_api::enabled;
use crate::model::{Length, LengthOrAuto, Node, NodeContent, NodeContentImage, NodeId, Path, Step, StepValue};
use crate::parsers::{parse_color, parse_length, parse_length_auto, parse_position, StringOrFloat};
use crate::pyinterface::insteps::ValueOrInSteps;
use pyo3::{FromPyObject, PyResult};
use crate::model::ImageManager;

#[derive(Debug, FromPyObject)]
enum PyStringOrFloat {
    Float(f32),
    String(String),
}

impl From<PyStringOrFloat> for StringOrFloat {
    fn from(value: PyStringOrFloat) -> Self {
        match value {
            PyStringOrFloat::Float(v) => StringOrFloat::Float(v),
            PyStringOrFloat::String(v) => StringOrFloat::String(v),
        }
    }
}

#[derive(Debug, FromPyObject)]
struct ImageContent {
    path: PathBuf,
    enable_steps: bool,
    shift_steps: Step,
}

#[derive(Debug, FromPyObject)]
enum Content {
    Image(ImageContent)
}

#[derive(Debug, FromPyObject)]
pub(crate) struct BoxConfig {
    pub bg_color: ValueOrInSteps<Option<String>>,
    pub x: ValueOrInSteps<Option<PyStringOrFloat>>,
    pub y: ValueOrInSteps<Option<PyStringOrFloat>>,
    pub width: ValueOrInSteps<Option<PyStringOrFloat>>,
    pub height: ValueOrInSteps<Option<PyStringOrFloat>>,
    pub row: ValueOrInSteps<bool>,
    pub reverse: ValueOrInSteps<bool>,
    pub p_left: ValueOrInSteps<PyStringOrFloat>,
    pub p_right: ValueOrInSteps<PyStringOrFloat>,
    pub p_top: ValueOrInSteps<PyStringOrFloat>,
    pub p_bottom: ValueOrInSteps<PyStringOrFloat>,
    pub m_left: ValueOrInSteps<PyStringOrFloat>,
    pub m_right: ValueOrInSteps<PyStringOrFloat>,
    pub m_top: ValueOrInSteps<PyStringOrFloat>,
    pub m_bottom: ValueOrInSteps<PyStringOrFloat>,
    pub z_level: ValueOrInSteps<i32>,
    pub name: String,
    pub debug_layout: Option<String>,
    pub content: ValueOrInSteps<Option<Content>>
}

fn pyparse_opt_length(obj: Option<PyStringOrFloat>) -> crate::Result<Option<Length>> {
    obj.map(|v| parse_length(v.into())).transpose()
    // match obj {
    //     None => Ok(None),
    //     Some(StringOrFloat::String(v)) => parse_length(&v).map(Some),
    //     Some(StringOrFloat::Float(value)) => Ok(Some(Length::Points { value }))
    // }
}

fn parse_len(obj: PyStringOrFloat) -> crate::Result<Length> {
    parse_length(obj.into())
}

fn parse_len_auto(obj: PyStringOrFloat) -> crate::Result<LengthOrAuto> {
    parse_length_auto(obj.into())
}

fn process_content(content: Content, image_manager: &mut ImageManager, n_steps: &mut Step) -> crate::Result<NodeContent> {
    Ok(match content {
        Content::Image(image) => {
            let loaded_image = image_manager.load_image(&image.path)?;
            if image.enable_steps {
                *n_steps = (*n_steps).max(loaded_image.n_steps() + image.shift_steps);
            }
            NodeContent::Image(NodeContentImage {
                loaded_image,
                enable_steps: image.enable_steps,
                shift_steps: image.shift_steps,
            })
        }
    })
}

impl BoxConfig {
    pub fn make_node(self, node_id: NodeId, parent_id: NodeId, image_manager: &mut ImageManager) -> PyResult<(Node, Step)> {
        let mut n_steps = 1;
        let mut n_steps2 = 1;
        let content = self.content.parse(&mut n_steps, |c| c.map(|c| process_content(c, image_manager, &mut n_steps2)).transpose())?;
        n_steps = n_steps.max(n_steps2);

        let bg_color = self
            .bg_color
            .parse(&mut n_steps, |v| v.as_deref().map(parse_color).transpose())?;
        let x = self.x.parse(&mut n_steps, |v| v.map(|v| parse_position(parent_id, v.into(), true)).transpose())?;
        let y = self.y.parse(&mut n_steps, |v| v.map(|v| parse_position(parent_id, v.into(), false)).transpose())?;
        let width = self.width.parse(&mut n_steps, pyparse_opt_length)?;
        let height = self.height.parse(&mut n_steps, pyparse_opt_length)?;
        let node = Node {
            node_id,
            name: self.name,
            show: StepValue::new_const(true),
            z_level: self.z_level.to_step_value(&mut n_steps),
            x,
            y,
            width,
            height,
            row: self.row.to_step_value(&mut n_steps),
            reverse: self.reverse.to_step_value(&mut n_steps),
            p_top: self.p_top.parse(&mut n_steps, parse_len)?,
            p_bottom: self.p_bottom.parse(&mut n_steps, parse_len)?,
            p_left: self.p_left.parse(&mut n_steps, parse_len)?,
            p_right: self.p_right.parse(&mut n_steps, parse_len)?,
            m_top: self.m_top.parse(&mut n_steps, parse_len_auto)?,
            m_bottom: self.m_bottom.parse(&mut n_steps, parse_len_auto)?,
            m_left: self.m_left.parse(&mut n_steps, parse_len_auto)?,
            m_right: self.m_right.parse(&mut n_steps, parse_len_auto)?,
            bg_color,
            content,
            debug_layout: self.debug_layout.as_deref().map(parse_color).transpose()?,
            children: Vec::new(),
        };
        Ok((node, n_steps))
    }
}
