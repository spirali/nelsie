use crate::model::{
    merge_stepped_styles, Color, LengthOrExpr, NodeContentText, ParsedText, PartialTextStyle,
    StyleMap, TextAlign,
};
use crate::model::{
    Length, LengthOrAuto, Node, NodeContent, NodeContentImage, NodeId, Resources, Step, StepValue,
};
use crate::parsers::step_parser::parse_steps;
use crate::parsers::{
    parse_length, parse_length_auto, parse_length_or_expr, parse_position, parse_styled_text,
    parse_styled_text_from_plain_text, run_syntax_highlighting, StyleOrName,
};
use crate::pyinterface::basictypes::{PyStringOrFloat, PyStringOrFloatOrExpr};
use crate::pyinterface::insteps::{InSteps, ValueOrInSteps};
use crate::pyinterface::textstyle::PyTextStyleOrName;
use std::collections::BTreeMap;

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
    Int(u32),
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
    text: ValueOrInSteps<String>,
    style1: Option<PyTextStyleOrName>,
    style2: Option<PyTextStyleOrName>,
    formatting_delimiters: Option<String>,
    text_align: u32,
    syntax_language: Option<String>,
    syntax_theme: Option<String>,
    parse_counters: bool,
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

fn pyparse_opt_length_or_expr(
    obj: Option<PyStringOrFloatOrExpr>,
) -> crate::Result<Option<LengthOrExpr>> {
    obj.map(|v| parse_length_or_expr(v.into())).transpose()
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

fn process_text_parsing(
    text: &str,
    resources: &Resources,
    formatting_delimiters: Option<&str>,
    syntax_language: Option<&str>,
    syntax_theme: Option<&str>,
    main_style: &StepValue<PartialTextStyle>,
    styles: &StyleMap,
) -> PyResult<ParsedText> {
    let mut parsed = if let Some(delimiters) = formatting_delimiters {
        if delimiters.chars().count() != 3 {
            return Err(PyValueError::new_err("Invalid delimiters, it has to be 3 char string (escape character, start of block, end of block)"));
        }
        let mut f = delimiters.chars();
        let esc_char = f.next().unwrap();
        let start_block = f.next().unwrap();
        let end_block = f.next().unwrap();
        parse_styled_text(text, esc_char, start_block, end_block)?
    } else {
        parse_styled_text_from_plain_text(text)
    };

    if let Some(language) = &syntax_language {
        let theme = syntax_theme
            .as_ref()
            .ok_or_else(|| PyValueError::new_err("Invalid theme"))?;
        run_syntax_highlighting(resources, &mut parsed, language, theme)?;
    }

    let styles = parsed
        .styles
        .into_iter()
        .map(|names| {
            names
                .into_iter()
                .try_fold(main_style.clone(), |s, style_or_name| {
                    Ok(match style_or_name {
                        StyleOrName::Name(name) => {
                            merge_stepped_styles(&s, styles.get_style(name)?)
                        }
                        StyleOrName::Style(style) => s.map(|x| x.merge(&style)),
                    })
                })
                .map(|s| s.map(|v| v.into_text_style().unwrap()))
        })
        .collect::<crate::Result<Vec<_>>>()?;
    Ok(ParsedText {
        styled_lines: parsed.styled_lines,
        styles,
        anchors: parsed.anchors,
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
            let default = styles.get_style("default")?;
            let mut main_style = if let Some(style) = text.style1 {
                resolve_style(nc_env.resources, default, style, styles, n_steps)?
            } else {
                default.clone()
            };
            if let Some(style) = text.style2 {
                main_style = resolve_style(nc_env.resources, &main_style, style, styles, n_steps)?
            };

            let parsed_text = text.text.parse(n_steps, |txt| {
                process_text_parsing(
                    &txt,
                    nc_env.resources,
                    text.formatting_delimiters.as_deref(),
                    text.syntax_language.as_deref(),
                    text.syntax_theme.as_deref(),
                    &main_style,
                    styles,
                )
            })?;

            let node_content = NodeContentText {
                parsed_text,
                text_align,
                default_font_size: main_style.map_ref(|s| s.size.unwrap()),
                default_line_spacing: main_style.map_ref(|s| s.line_spacing.unwrap()),
                parse_counters: text.parse_counters,
            };

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

fn parse_align_items(value: Option<&str>) -> crate::Result<Option<AlignItems>> {
    value
        .map(|v| match v {
            "start" => Ok(AlignItems::Start),
            "end" => Ok(AlignItems::End),
            "flex-start" => Ok(AlignItems::FlexStart),
            "flex-end" => Ok(AlignItems::FlexEnd),
            "center" => Ok(AlignItems::Center),
            "stretch" => Ok(AlignItems::Stretch),
            "baseline" => Ok(AlignItems::Baseline),
            _ => Err(NelsieError::parsing_err("Invalid AlignItems")),
        })
        .transpose()
}

fn parse_align_content(value: Option<&str>) -> crate::Result<Option<AlignContent>> {
    value
        .map(|v| match v {
            "start" => Ok(AlignContent::Start),
            "end" => Ok(AlignContent::End),
            "flex-start" => Ok(AlignContent::FlexStart),
            "flex-end" => Ok(AlignContent::FlexEnd),
            "center" => Ok(AlignContent::Center),
            "stretch" => Ok(AlignContent::Stretch),
            "space-between" => Ok(AlignContent::SpaceBetween),
            "space-evenly" => Ok(AlignContent::SpaceEvenly),
            "space-around" => Ok(AlignContent::SpaceAround),
            x => Err(NelsieError::parsing_err(format!(
                "Invalid AlignContent '{x}'"
            ))),
        })
        .transpose()
}

fn show_to_bool_steps(show: Show, n_steps: &mut Step) -> PyResult<StepValue<bool>> {
    Ok(match show {
        Show::Bool(value) => StepValue::new_const(value),
        Show::Int(value) => {
            *n_steps = (*n_steps).max(value);
            let mut map = BTreeMap::new();
            map.insert(value, true);
            map.insert(value + 1, false);
            StepValue::new_map(map)
        }
        Show::StringDef(s) => {
            let (steps, n) = parse_steps(&s)
                .ok_or_else(|| PyValueError::new_err(format!("Invalid show definition: {s}")))?;
            *n_steps = (*n_steps).max(n);
            steps
        }
        Show::InSteps(in_steps) => in_steps.into_step_value(n_steps),
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn make_node(
    new_node_id: NodeId,
    nc_env: &mut NodeCreationEnv,
    styles: Arc<StyleMap>,
    active: Show,
    show: Show,
    bg_color: ValueOrInSteps<Option<String>>,
    x: ValueOrInSteps<Option<PyStringOrFloatOrExpr>>,
    y: ValueOrInSteps<Option<PyStringOrFloatOrExpr>>,
    width: ValueOrInSteps<Option<PyStringOrFloatOrExpr>>,
    height: ValueOrInSteps<Option<PyStringOrFloatOrExpr>>,
    border_radius: ValueOrInSteps<f32>,
    row: ValueOrInSteps<bool>,
    reverse: ValueOrInSteps<bool>,
    flex_wrap: ValueOrInSteps<&str>,
    flex_grow: ValueOrInSteps<f32>,
    flex_shrink: ValueOrInSteps<f32>,

    align_items: ValueOrInSteps<Option<&str>>,
    align_self: ValueOrInSteps<Option<&str>>,
    justify_self: ValueOrInSteps<Option<&str>>,
    align_content: ValueOrInSteps<Option<&str>>,
    justify_content: ValueOrInSteps<Option<&str>>,
    gap: ValueOrInSteps<(PyStringOrFloat, PyStringOrFloat)>,

    p_left: ValueOrInSteps<PyStringOrFloat>,
    p_right: ValueOrInSteps<PyStringOrFloat>,
    p_top: ValueOrInSteps<PyStringOrFloat>,
    p_bottom: ValueOrInSteps<PyStringOrFloat>,
    m_left: ValueOrInSteps<PyStringOrFloat>,
    m_right: ValueOrInSteps<PyStringOrFloat>,
    m_top: ValueOrInSteps<PyStringOrFloat>,
    m_bottom: ValueOrInSteps<PyStringOrFloat>,
    z_level: ValueOrInSteps<i32>,
    name: String,
    debug_layout: Option<String>,
    replace_steps: Option<BTreeMap<Step, Step>>,
    content: Option<Content>,
) -> PyResult<(Node, Step)> {
    let mut n_steps = 1;
    let mut n_steps2 = 1;
    let content = content
        .map(|c| process_content(c, nc_env, &styles, &mut n_steps2))
        .transpose()?;
    n_steps = n_steps.max(n_steps2);
    let flex_wrap = flex_wrap.parse(&mut n_steps, |f| match f {
        "nowrap" => Ok(FlexWrap::NoWrap),
        "wrap" => Ok(FlexWrap::Wrap),
        "wrap-reverse" => Ok(FlexWrap::WrapReverse),
        _ => Err(PyValueError::new_err("Invalid wrap value")),
    })?;
    let bg_color = bg_color.parse(&mut n_steps, |v| {
        v.as_deref().map(Color::from_str).transpose()
    })?;
    let x = x.parse(&mut n_steps, |v| {
        v.map(|v| parse_position(v.into(), true)).transpose()
    })?;
    let y = y.parse(&mut n_steps, |v| {
        v.map(|v| parse_position(v.into(), false)).transpose()
    })?;
    let width = width.parse(&mut n_steps, pyparse_opt_length_or_expr)?;
    let height = height.parse(&mut n_steps, pyparse_opt_length_or_expr)?;
    let node = Node {
        node_id: new_node_id,
        replace_steps: replace_steps.unwrap_or_default(),
        name,
        active: show_to_bool_steps(active, &mut n_steps)?,
        show: show_to_bool_steps(show, &mut n_steps)?,
        z_level: z_level.into_step_value(&mut n_steps),
        x,
        y,
        width,
        height,
        border_radius: border_radius.into_step_value(&mut n_steps),
        row: row.into_step_value(&mut n_steps),
        reverse: reverse.into_step_value(&mut n_steps),
        flex_wrap,
        flex_grow: flex_grow.into_step_value(&mut n_steps),
        flex_shrink: flex_shrink.into_step_value(&mut n_steps),
        align_items: align_items.parse(&mut n_steps, parse_align_items)?,
        align_self: align_self.parse(&mut n_steps, parse_align_items)?,
        justify_self: justify_self.parse(&mut n_steps, parse_align_items)?,
        align_content: align_content.parse(&mut n_steps, parse_align_content)?,
        justify_content: justify_content.parse(&mut n_steps, parse_align_content)?,
        gap: gap.parse(&mut n_steps, |(w, h)| {
            crate::Result::Ok((parse_len(w)?, parse_len(h)?))
        })?,
        p_top: p_top.parse(&mut n_steps, parse_len)?,
        p_bottom: p_bottom.parse(&mut n_steps, parse_len)?,
        p_left: p_left.parse(&mut n_steps, parse_len)?,
        p_right: p_right.parse(&mut n_steps, parse_len)?,
        m_top: m_top.parse(&mut n_steps, parse_len_auto)?,
        m_bottom: m_bottom.parse(&mut n_steps, parse_len_auto)?,
        m_left: m_left.parse(&mut n_steps, parse_len_auto)?,
        m_right: m_right.parse(&mut n_steps, parse_len_auto)?,
        bg_color,
        content,
        debug_layout: debug_layout.as_deref().map(Color::from_str).transpose()?,
        children: Vec::new(),
        styles,
    };
    Ok((node, n_steps))
}
