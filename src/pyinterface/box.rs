use crate::model::{
    merge_stepped_styles, Color, NodeContentText, PartialTextStyle, StyleMap, TextAlign,
};
use crate::model::{
    Length, LengthOrAuto, Node, NodeContent, NodeContentImage, NodeId, Resources, Step, StepValue,
};
use crate::parsers::step_parser::parse_steps;
use crate::parsers::{
    parse_length, parse_length_auto, parse_position, parse_styled_text,
    parse_styled_text_from_plain_text, run_syntax_highlighting,
};
use crate::pyinterface::basictypes::{PyStringOrFloat, PyStringOrFloatOrExpr};
use crate::pyinterface::insteps::{InSteps, ValueOrInSteps};
use crate::pyinterface::textstyle::PyTextStyleOrName;

use pyo3::exceptions::PyValueError;
use pyo3::{FromPyObject, PyAny, PyResult};

use crate::common::error::NelsieError;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use taffy::prelude::{AlignContent, AlignItems};
use taffy::style::FlexWrap;

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
    style1: Option<PyTextStyleOrName>,
    style2: Option<PyTextStyleOrName>,
    formatting_delimiters: Option<String>,
    text_align: u32,
    syntax_language: Option<String>,
    syntax_theme: Option<String>,
}

#[derive(Debug)]
pub(crate) enum Content {
    Text(TextContent),
    Image(ImageContent),
}

impl<'py> FromPyObject<'py> for Content {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        Ok(if ob.hasattr("text")? {
            Content::Text(ob.extract()?)
        } else {
            Content::Image(ob.extract()?)
        })
    }
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
    pub flex_wrap: ValueOrInSteps<u32>,
    pub flex_grow: ValueOrInSteps<f32>,
    pub flex_shrink: ValueOrInSteps<f32>,

    pub align_items: ValueOrInSteps<Option<u32>>,
    pub align_self: ValueOrInSteps<Option<u32>>,
    pub justify_self: ValueOrInSteps<Option<u32>>,
    pub align_content: ValueOrInSteps<Option<u32>>,
    pub justify_content: ValueOrInSteps<Option<u32>>,
    pub gap: ValueOrInSteps<(PyStringOrFloat, PyStringOrFloat)>,

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

fn resolve_style(
    resources: &Resources,
    original: &StepValue<PartialTextStyle>,
    style_or_name: PyTextStyleOrName,
    styles: &StyleMap,
    n_steps: &mut Step,
) -> crate::Result<StepValue<PartialTextStyle>> {
    Ok(match style_or_name {
        PyTextStyleOrName::Name(name) => merge_stepped_styles(original, styles.get_style(&name)?),
        PyTextStyleOrName::Style(style) => merge_stepped_styles(
            original,
            &style.parse(n_steps, |s| s.into_partial_style(resources))?,
        ),
    })
}

fn process_content(
    content: Content,
    nc_env: &mut NodeCreationEnv,
    styles: &StyleMap,
    n_steps: &mut Step,
) -> PyResult<NodeContent> {
    Ok(match content {
        Content::Text(text) => {
            let text_align = match text.text_align {
                0 => TextAlign::Start,
                1 => TextAlign::Center,
                2 => TextAlign::End,
                _ => return Err(PyValueError::new_err("Invalid text align")),
            };
            let parsed = if let Some(delimiters) = text.formatting_delimiters {
                if delimiters.chars().count() != 3 {
                    return Err(PyValueError::new_err("Invalid delimiters, it has to be 3 char string (escape character, start of block, end of block)"));
                }
                let mut f = delimiters.chars();
                let esc_char = f.next().unwrap();
                let start_block = f.next().unwrap();
                let end_block = f.next().unwrap();
                parse_styled_text(&text.text, esc_char, start_block, end_block)?
            } else {
                parse_styled_text_from_plain_text(&text.text)
            };
            let default = styles.get_style("default")?;
            let mut main_style = if let Some(style) = text.style1 {
                resolve_style(nc_env.resources, default, style, styles, n_steps)?
            } else {
                default.clone()
            };
            if let Some(style) = text.style2 {
                main_style = resolve_style(nc_env.resources, &main_style, style, styles, n_steps)?
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

            let mut node_content = NodeContentText {
                styled_lines: parsed.styled_lines,
                styles,
                text_align,
                default_font_size: main_style.map_ref(|s| s.size.unwrap()),
                default_line_spacing: main_style.map_ref(|s| s.line_spacing.unwrap()),
            };

            if let Some(language) = text.syntax_language {
                let theme = text
                    .syntax_theme
                    .ok_or_else(|| PyValueError::new_err("Invalid theme"))?;
                run_syntax_highlighting(nc_env.resources, &mut node_content, &language, &theme)?;
            }
            NodeContent::Text(node_content)
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

fn parse_align_items(value: Option<u32>) -> crate::Result<Option<AlignItems>> {
    value
        .map(|v| match v {
            0 => Ok(AlignItems::Start),
            1 => Ok(AlignItems::End),
            2 => Ok(AlignItems::FlexStart),
            3 => Ok(AlignItems::FlexEnd),
            4 => Ok(AlignItems::Center),
            5 => Ok(AlignItems::Stretch),
            10 => Ok(AlignItems::Baseline),
            20..=22 => Err(NelsieError::parsing_err(
                "SpaceBetween, SpaceEvenly, SpaceAround values cannot be used in this context",
            )),
            _ => Err(NelsieError::parsing_err("Invalid AlignItems")),
        })
        .transpose()
}

fn parse_align_content(value: Option<u32>) -> crate::Result<Option<AlignContent>> {
    value
        .map(|v| match v {
            0 => Ok(AlignContent::Start),
            1 => Ok(AlignContent::End),
            2 => Ok(AlignContent::FlexStart),
            3 => Ok(AlignContent::FlexEnd),
            4 => Ok(AlignContent::Center),
            5 => Ok(AlignContent::Stretch),
            10 => Err(NelsieError::parsing_err(
                "Baseline value cannot be used in this context",
            )),
            20 => Ok(AlignContent::SpaceBetween),
            21 => Ok(AlignContent::SpaceEvenly),
            22 => Ok(AlignContent::SpaceAround),
            _ => Err(NelsieError::parsing_err("Invalid AlignContent")),
        })
        .transpose()
}

impl BoxConfig {
    pub fn make_node(
        self,
        new_node_id: NodeId,
        nc_env: &mut NodeCreationEnv,
        styles: Arc<StyleMap>,
        content: Option<Content>,
    ) -> PyResult<(Node, Step)> {
        let mut n_steps = 1;
        let mut n_steps2 = 1;
        let content = content
            .map(|c| process_content(c, nc_env, &styles, &mut n_steps2))
            .transpose()?;
        n_steps = n_steps.max(n_steps2);
        let flex_wrap = self.flex_wrap.parse(&mut n_steps, |f| match f {
            0 => Ok(FlexWrap::NoWrap),
            1 => Ok(FlexWrap::Wrap),
            2 => Ok(FlexWrap::WrapReverse),
            _ => Err(PyValueError::new_err("Invalid wrap value")),
        })?;
        let bg_color = self.bg_color.parse(&mut n_steps, |v| {
            v.as_deref().map(Color::from_str).transpose()
        })?;
        let x = self.x.parse(&mut n_steps, |v| {
            v.map(|v| parse_position(v.into(), true)).transpose()
        })?;
        let y = self.y.parse(&mut n_steps, |v| {
            v.map(|v| parse_position(v.into(), false)).transpose()
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
            Show::InSteps(in_steps) => in_steps.into_step_value(&mut n_steps),
        };
        let width = self.width.parse(&mut n_steps, pyparse_opt_length)?;
        let height = self.height.parse(&mut n_steps, pyparse_opt_length)?;
        let node = Node {
            node_id: new_node_id,
            name: self.name,
            show,
            z_level: self.z_level.into_step_value(&mut n_steps),
            x,
            y,
            width,
            height,
            row: self.row.into_step_value(&mut n_steps),
            reverse: self.reverse.into_step_value(&mut n_steps),
            flex_wrap,
            flex_grow: self.flex_grow.into_step_value(&mut n_steps),
            flex_shrink: self.flex_shrink.into_step_value(&mut n_steps),
            align_items: self.align_items.parse(&mut n_steps, parse_align_items)?,
            align_self: self.align_self.parse(&mut n_steps, parse_align_items)?,
            justify_self: self.justify_self.parse(&mut n_steps, parse_align_items)?,
            align_content: self
                .align_content
                .parse(&mut n_steps, parse_align_content)?,
            justify_content: self
                .justify_content
                .parse(&mut n_steps, parse_align_content)?,
            gap: self.gap.parse(&mut n_steps, |(w, h)| {
                crate::Result::Ok((parse_len(w)?, parse_len(h)?))
            })?,
            p_top: self.p_top.parse(&mut n_steps, parse_len)?,
            p_bottom: self.p_bottom.parse(&mut n_steps, parse_len)?,
            p_left: self.p_left.parse(&mut n_steps, parse_len)?,
            p_right: self.p_right.parse(&mut n_steps, parse_len)?,
            m_top: self.m_top.parse(&mut n_steps, parse_len_auto)?,
            m_bottom: self.m_bottom.parse(&mut n_steps, parse_len_auto)?,
            m_left: self.m_left.parse(&mut n_steps, parse_len_auto)?,
            m_right: self.m_right.parse(&mut n_steps, parse_len_auto)?,
            bg_color,
            content: StepValue::new_const(content),
            debug_layout: self
                .debug_layout
                .as_deref()
                .map(Color::from_str)
                .transpose()?,
            children: Vec::new(),
            styles,
        };
        Ok((node, n_steps))
    }
}
