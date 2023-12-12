use crate::model::{merge_stepped_styles, NodeContentText, StyleMap};
use crate::model::{
    Length, LengthOrAuto, Node, NodeContent, NodeContentImage, NodeId, Resources, Step, StepValue,
};
use crate::parsers::step_parser::parse_steps;
use crate::parsers::{
    parse_color, parse_length, parse_length_auto, parse_position, parse_styled_text,
};
use crate::pyinterface::basictypes::{PyStringOrFloat, PyStringOrFloatOrExpr};
use crate::pyinterface::insteps::{InSteps, ValueOrInSteps};
use crate::pyinterface::textstyle::PyTextStyleOrName;

use pyo3::exceptions::PyValueError;
use pyo3::{FromPyObject, PyResult};

use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, FromPyObject)]
pub(crate) enum Show {
    Bool(bool),
    StringDef(String),
    InSteps(InSteps<bool>),
}

#[derive(Debug, FromPyObject)]
pub(crate) struct ImageContent {
    path: PathBuf,
    enable_steps: bool,
    shift_steps: Step,
}

#[derive(Debug, FromPyObject)]
pub(crate) struct TextContent {
    text: String,
    style: PyTextStyleOrName,
    formatting_delimiters: String,
}

#[derive(Debug, FromPyObject)]
pub(crate) enum Content {
    Text(TextContent),
    Image(ImageContent),
}

#[derive(Debug, FromPyObject)]
pub(crate) struct BoxConfig {
    pub show: Show,
    pub bg_color: ValueOrInSteps<Option<String>>,
    pub x: ValueOrInSteps<Option<PyStringOrFloatOrExpr>>,
    pub y: ValueOrInSteps<Option<PyStringOrFloatOrExpr>>,
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
    pub content: ValueOrInSteps<Option<Content>>,
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

pub(crate) struct NodeCreationEnv<'a> {
    pub resources: &'a mut Resources,
}

fn process_content(
    content: Content,
    nc_env: &mut NodeCreationEnv,
    styles: &StyleMap,
    n_steps: &mut Step,
) -> PyResult<NodeContent> {
    Ok(match content {
        Content::Text(text) => {
            if text.formatting_delimiters.chars().count() != 3 {
                return Err(PyValueError::new_err("Invalid delimiters, it has to be 3 char string (escape character, start of block, end of block)"));
            }
            let mut f = text.formatting_delimiters.chars();
            let esc_char = f.next().unwrap();
            let start_block = f.next().unwrap();
            let end_block = f.next().unwrap();

            let parsed = parse_styled_text(&text.text, esc_char, start_block, end_block)?;
            let default = styles.get_style("default")?;
            let main_style = match text.style {
                PyTextStyleOrName::Name(name) if name == "default" => default.clone(),
                PyTextStyleOrName::Name(name) => {
                    merge_stepped_styles(default, styles.get_style(&name)?)
                }
                PyTextStyleOrName::Style(style) => merge_stepped_styles(
                    default,
                    &style.parse(n_steps, |s| s.to_partial_style(nc_env.resources))?,
                ),
            };
            let styles = parsed
                .styles
                .into_iter()
                .map(|names| {
                    names
                        .into_iter()
                        .try_fold(main_style.clone(), |s, name| {
                            Ok(merge_stepped_styles(&s, styles.get_style(name)?))
                        })
                        .map(|s| s.map(|v| v.into_text_style().unwrap()))
                })
                .collect::<crate::Result<Vec<_>>>()?;

            NodeContent::Text(NodeContentText {
                styled_lines: parsed.styled_lines,
                styles,
                default_font_size: main_style.map_ref(|s| s.size.unwrap()),
                default_line_spacing: main_style.map_ref(|s| s.line_spacing.unwrap()),
            })
        }
        Content::Image(image) => {
            let loaded_image = nc_env.resources.image_manager.load_image(&image.path)?;
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
    pub fn make_node(
        self,
        new_node_id: NodeId,
        nc_env: &mut NodeCreationEnv,
        styles: Arc<StyleMap>,
    ) -> PyResult<(Node, Step)> {
        let mut n_steps = 1;
        let mut n_steps2 = 1;
        let content = self.content.parse(&mut n_steps, |c| {
            c.map(|c| process_content(c, nc_env, &styles, &mut n_steps2))
                .transpose()
        })?;
        n_steps = n_steps.max(n_steps2);

        let bg_color = self
            .bg_color
            .parse(&mut n_steps, |v| v.as_deref().map(parse_color).transpose())?;
        let x = self.x.parse(&mut n_steps, |v| {
            v.map(|v| parse_position(&v.into(), true)).transpose()
        })?;
        let y = self.y.parse(&mut n_steps, |v| {
            v.map(|v| parse_position(&v.into(), false)).transpose()
        })?;
        let show = match self.show {
            Show::Bool(value) => StepValue::new_const(value),
            Show::StringDef(s) => {
                let (steps, n) = parse_steps(&s).ok_or_else(|| {
                    PyValueError::new_err(format!("Invalid show definition: {s}"))
                })?;
                n_steps = n_steps.max(n);
                steps
            }
            Show::InSteps(in_steps) => in_steps.to_step_value(&mut n_steps),
        };
        let width = self.width.parse(&mut n_steps, pyparse_opt_length)?;
        let height = self.height.parse(&mut n_steps, pyparse_opt_length)?;
        let node = Node {
            node_id: new_node_id,
            name: self.name,
            show,
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
            styles,
        };
        Ok((node, n_steps))
    }
}
