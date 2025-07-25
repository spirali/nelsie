use crate::model::{
    merge_stepped_styles, LengthOrExpr, LoadedImage, LoadedImageData, NodeContentText,
    NodeContentVideo, PartialTextStyle, Step, StepIndex, StepSet, StyleMap, StyledRange,
    StyledText, TextAlign, Video,
};
use crate::model::{
    Length, LengthOrAuto, Node, NodeContent, NodeContentImage, NodeId, Resources, StepValue,
};
use crate::parsers::step_parser::parse_steps_with_keywords;
use crate::parsers::{
    parse_grid_position_item, parse_grid_template_item, parse_length, parse_length_auto,
    parse_length_or_expr, parse_position, parse_styled_text, parse_styled_text_from_plain_text,
    run_syntax_highlighting, StyleOrName,
};
use crate::pyinterface::basictypes::{PyStringOrFloat, PyStringOrFloatOrExpr, PyStringOrI16};
use crate::pyinterface::insteps::{InSteps, ValueOrInSteps};
use crate::pyinterface::textstyle::PyTextStyleOrName;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Deref;

use pyo3::exceptions::{PyException, PyValueError};
use pyo3::types::PyAnyMethods;
use pyo3::{Bound, FromPyObject, PyAny, PyResult};

use crate::common::error::NelsieError;
use crate::common::Color;
use itertools::Itertools;
use pyo3::pybacked::PyBackedStr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use taffy::prelude::{AlignContent, AlignItems};
use taffy::style::FlexWrap;
use taffy::{GridPlacement, Line, NonRepeatedTrackSizingFunction};

#[derive(Debug, FromPyObject)]
pub(crate) enum Show {
    Bool(bool),
    Int(u32),
    StringDef(String),
    VecInt(Vec<u32>),
    InSteps(InSteps<bool>),
}

#[derive(Debug, FromPyObject)]
pub(crate) enum PathOrData {
    Path(PathBuf),
    Data(Vec<u8>, String),
}

#[derive(Debug, FromPyObject)]
pub(crate) struct ImageContent {
    path_or_data: ValueOrInSteps<Option<PathOrData>>,
    enable_steps: bool,
    shift_steps: StepIndex,
}

#[derive(Debug, FromPyObject)]
pub(crate) struct VideoContent {
    path: PathBuf,
    cover_image: Option<PathBuf>,
    data_type: String,
    show_controls: bool,
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
    Video(VideoContent),
}

#[derive(Debug, FromPyObject)]
pub(crate) enum PyGridPosition {
    Single(PyStringOrI16),
    Pair((PyStringOrI16, PyStringOrI16)),
}

impl<'py> FromPyObject<'py> for Content {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(if ob.hasattr("text")? {
            Content::Text(ob.extract()?)
        } else if ob.hasattr("enable_steps")? {
            Content::Image(ob.extract()?)
        } else {
            Content::Video(ob.extract()?)
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
    resources: &mut Resources,
    original: &StepValue<PartialTextStyle>,
    style_or_name: PyTextStyleOrName,
    styles: &StyleMap,
    steps: &mut StepSet,
) -> crate::Result<StepValue<PartialTextStyle>> {
    Ok(match style_or_name {
        PyTextStyleOrName::Name(name) => merge_stepped_styles(original, styles.get_style(&name)?),
        PyTextStyleOrName::Style(style) => merge_stepped_styles(
            original,
            &style.parse(steps, |s| s.into_partial_style(resources))?,
        ),
    })
}

#[allow(clippy::too_many_arguments)]
fn process_text_parsing(
    text: StepValue<String>,
    resources: &Resources,
    formatting_delimiters: Option<&str>,
    syntax_language: Option<&str>,
    syntax_theme: Option<&str>,
    main_style: &StepValue<PartialTextStyle>,
    styles: &StyleMap,
    text_align: TextAlign,
) -> PyResult<StepValue<StyledText>> {
    let parsed = text.try_map_ref(|s| {
        let mut parsed = if let Some(delimiters) = formatting_delimiters {
            if delimiters.chars().count() != 3 {
                return Err(PyValueError::new_err("Invalid delimiters, it has to be 3 char string (escape character, start of block, end of block)"));
            }
            let mut f = delimiters.chars();
            let esc_char = f.next().unwrap();
            let start_block = f.next().unwrap();
            let end_block = f.next().unwrap();
            parse_styled_text(s, esc_char, start_block, end_block)?
        } else {
            parse_styled_text_from_plain_text(s)
        };
        if let Some(language) = &syntax_language {
            let theme = syntax_theme
                .as_ref()
                .ok_or_else(|| PyValueError::new_err("Invalid syntax highlight theme"))?;
            run_syntax_highlighting(resources, &mut parsed, language, theme)?;
        }

        let styles = parsed
            .styles
            .into_iter()
            .map(|style| {
                Ok((
                    style.start,
                    style.end,
                    match style.style {
                        StyleOrName::Name(name) =>
                            Cow::Borrowed(styles.get_style(name)?),
                        StyleOrName::Style(style) => Cow::Owned(StepValue::Const(style)),
                    }
                ))
            })
            .collect::<crate::Result<Vec<_>>>()?;
        Ok((parsed.text, styles, parsed.anchors))
    }
    )?;

    let mut steps: BTreeSet<&Step> = parsed.steps().collect();
    steps.extend(
        parsed
            .values()
            .flat_map(|v| v.1.iter().flat_map(|s| s.2.steps())),
    );
    steps.extend(main_style.steps());

    Ok(if steps.is_empty() {
        let (text, styles, anchors) = parsed.get_const().unwrap();
        let main_style = main_style
            .clone()
            .get_const()
            .unwrap()
            .into_text_style()
            .unwrap();
        let styles = styles
            .into_iter()
            .map(|(start, end, s)| StyledRange {
                start,
                end,
                style: s.into_owned().get_const().unwrap(),
            })
            .collect_vec();
        StepValue::Const(StyledText {
            text,
            main_style,
            styles,
            anchors,
            text_align,
        })
    } else {
        let mut map = BTreeMap::new();
        for step in steps {
            let (text, styles, anchors) = parsed.at_step(step);
            let main_style = main_style.at_step(step).clone().into_text_style().unwrap();
            let styles = styles
                .iter()
                .map(|(start, end, s)| StyledRange {
                    start: *start,
                    end: *end,
                    style: s.at_step(step).clone(),
                })
                .collect_vec();
            map.insert(
                step.clone(),
                StyledText {
                    text: text.clone(),
                    main_style: main_style.clone(),
                    styles,
                    anchors: anchors.clone(),
                    text_align,
                },
            );
        }
        StepValue::new_map(map)
    })
}

fn process_content(
    content: Content,
    nc_env: &mut NodeCreationEnv,
    styles: &StyleMap,
    steps: &mut StepSet,
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
                resolve_style(nc_env.resources, default, style, styles, steps)?
            } else {
                default.clone()
            };
            if let Some(style) = text.style2 {
                main_style = resolve_style(nc_env.resources, &main_style, style, styles, steps)?
            };
            let text_str = text.text.into_step_value(steps);
            let styled_text = process_text_parsing(
                text_str,
                nc_env.resources,
                text.formatting_delimiters.as_deref(),
                text.syntax_language.as_deref(),
                text.syntax_theme.as_deref(),
                &main_style,
                styles,
                text_align,
            )?;

            let node_content = NodeContentText {
                styled_text,
                parse_counters: text.parse_counters,
            };

            NodeContent::Text(node_content)
        }
        Content::Image(image) => {
            let loaded_image: StepValue<Option<Arc<LoadedImage>>> =
                image.path_or_data.parse(steps, |path_or_data| {
                    crate::Result::Ok(if let Some(path_or_data) = path_or_data {
                        Some(match path_or_data {
                            PathOrData::Path(path) => nc_env
                                .resources
                                .image_manager
                                .load_image(&path, nc_env.resources.font_db.as_ref().unwrap())?,
                            PathOrData::Data(data, format) => match format.as_str() {
                                "png" | "jpeg" => {
                                    nc_env.resources.image_manager.load_raster_image(data)?
                                }
                                "svg" => nc_env.resources.image_manager.load_svg_image(
                                    data,
                                    nc_env.resources.font_db.as_ref().unwrap(),
                                )?,
                                _ => {
                                    return Err(NelsieError::generic_err(format!(
                                        "Unknown format: {format}"
                                    )))
                                }
                            },
                        })
                    } else {
                        None
                    })
                })?;
            if image.enable_steps {
                for img in loaded_image.values().flatten() {
                    img.update_steps(steps, image.shift_steps);
                }
            }
            NodeContent::Image(NodeContentImage {
                loaded_image,
                enable_steps: image.enable_steps,
                shift_steps: image.shift_steps,
            })
        }
        Content::Video(video) => {
            if !video.path.exists() {
                return Err(PyException::new_err(format!(
                    "Video file does not exist: {}",
                    video.path.display()
                )));
            }
            if !video.path.is_file() {
                return Err(PyException::new_err(format!(
                    "Path {} is not a file",
                    video.path.display()
                )));
            }
            let cover_image = video
                .cover_image
                .map(|path| {
                    let image = nc_env
                        .resources
                        .image_manager
                        .load_image(&path, nc_env.resources.font_db.as_ref().unwrap())
                        .and_then(|image| match image.data {
                            LoadedImageData::Png(_) | LoadedImageData::Jpeg(_) => Ok(image),
                            LoadedImageData::Svg(_) | LoadedImageData::Ora(_) => {
                                Err(NelsieError::generic_err(
                                    "Invalid format (only formats png and jpeg are supported)",
                                ))
                            }
                        });
                    image
                })
                .transpose()
                .map_err(|e| PyException::new_err(format!("cover image: {e}")))?;
            NodeContent::Video(NodeContentVideo {
                video: Arc::new(Video {
                    path: video.path,
                    cover_image,
                    data_type: video.data_type,
                    show_controls: video.show_controls,
                }),
            })
        }
    })
}

fn parse_grid_template(
    value: Vec<PyStringOrFloat>,
) -> crate::Result<Vec<NonRepeatedTrackSizingFunction>> {
    value
        .into_iter()
        .map(|x| parse_grid_template_item(x.into()))
        .collect()
}

fn parse_grid_position(value: PyGridPosition) -> crate::Result<Line<GridPlacement>> {
    match value {
        PyGridPosition::Single(v) => {
            let v = parse_grid_position_item(v.into())?;
            Ok(Line {
                start: v,
                end: GridPlacement::Auto,
            })
        }
        PyGridPosition::Pair((start, end)) => {
            let start = parse_grid_position_item(start.into())?;
            let end = parse_grid_position_item(end.into())?;
            Ok(Line { start, end })
        }
    }
}

fn parse_align_items(value: Option<PyBackedStr>) -> crate::Result<Option<AlignItems>> {
    value
        .map(|v| match v.deref() {
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

fn parse_align_content(value: Option<PyBackedStr>) -> crate::Result<Option<AlignContent>> {
    value
        .map(|v| match v.deref() {
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

fn show_to_bool_steps(show: Show, steps: &mut StepSet) -> PyResult<StepValue<bool>> {
    Ok(match show {
        Show::Bool(value) => StepValue::new_const(value),
        Show::Int(value) => {
            steps.insert(Step::from_int(value));
            let mut map = BTreeMap::new();
            map.insert(Step::from_int(value), true);
            map.insert(Step::from_int(value + 1), false);
            StepValue::new_map(map)
        }
        Show::VecInt(value) => {
            let s = Step::from_vec(value);
            steps.insert(s.clone());
            let mut map = BTreeMap::new();
            map.insert(s.next(), false);
            map.insert(s, true);
            StepValue::new_map(map)
        }
        Show::StringDef(s) => parse_steps_with_keywords(&s, steps)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid show definition: {s}")))?,
        Show::InSteps(in_steps) => in_steps.into_step_value(steps),
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn make_node(
    steps: &mut StepSet,
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
    flex_wrap: ValueOrInSteps<PyBackedStr>,
    flex_grow: ValueOrInSteps<f32>,
    flex_shrink: ValueOrInSteps<f32>,

    align_items: ValueOrInSteps<Option<PyBackedStr>>,
    align_self: ValueOrInSteps<Option<PyBackedStr>>,
    justify_self: ValueOrInSteps<Option<PyBackedStr>>,
    align_content: ValueOrInSteps<Option<PyBackedStr>>,
    justify_content: ValueOrInSteps<Option<PyBackedStr>>,
    gap: ValueOrInSteps<(PyStringOrFloat, PyStringOrFloat)>,

    grid_template_rows: ValueOrInSteps<Vec<PyStringOrFloat>>,
    grid_template_columns: ValueOrInSteps<Vec<PyStringOrFloat>>,
    grid_row: ValueOrInSteps<PyGridPosition>,
    grid_column: ValueOrInSteps<PyGridPosition>,

    p_left: ValueOrInSteps<PyStringOrFloat>,
    p_right: ValueOrInSteps<PyStringOrFloat>,
    p_top: ValueOrInSteps<PyStringOrFloat>,
    p_bottom: ValueOrInSteps<PyStringOrFloat>,
    m_left: ValueOrInSteps<PyStringOrFloat>,
    m_right: ValueOrInSteps<PyStringOrFloat>,
    m_top: ValueOrInSteps<PyStringOrFloat>,
    m_bottom: ValueOrInSteps<PyStringOrFloat>,
    z_level: ValueOrInSteps<i32>,
    url: ValueOrInSteps<Option<String>>,
    name: String,
    debug_layout: Option<String>,
    replace_steps: Option<BTreeMap<Step, Step>>,
    content: Option<Content>,
) -> PyResult<Node> {
    let content = content
        .map(|c| process_content(c, nc_env, &styles, steps))
        .transpose()?;
    let flex_wrap = flex_wrap.parse(steps, |f| match f.deref() {
        "nowrap" => Ok(FlexWrap::NoWrap),
        "wrap" => Ok(FlexWrap::Wrap),
        "wrap-reverse" => Ok(FlexWrap::WrapReverse),
        _ => Err(PyValueError::new_err("Invalid wrap value")),
    })?;
    let bg_color = bg_color.parse(steps, |v| v.as_deref().map(Color::from_str).transpose())?;
    let x = x.parse(steps, |v| {
        v.map(|v| parse_position(v.into(), true)).transpose()
    })?;
    let y = y.parse(steps, |v| {
        v.map(|v| parse_position(v.into(), false)).transpose()
    })?;
    let width = width.parse(steps, pyparse_opt_length_or_expr)?;
    let height = height.parse(steps, pyparse_opt_length_or_expr)?;
    let node = Node {
        node_id: new_node_id,
        replace_steps: replace_steps.unwrap_or_default(),
        name,
        active: show_to_bool_steps(active, steps)?,
        show: show_to_bool_steps(show, steps)?,
        z_level: z_level.into_step_value(steps),
        x,
        y,
        width,
        height,
        border_radius: border_radius.into_step_value(steps),
        row: row.into_step_value(steps),
        reverse: reverse.into_step_value(steps),
        flex_wrap,
        flex_grow: flex_grow.into_step_value(steps),
        flex_shrink: flex_shrink.into_step_value(steps),
        align_items: align_items.parse(steps, parse_align_items)?,
        align_self: align_self.parse(steps, parse_align_items)?,
        justify_self: justify_self.parse(steps, parse_align_items)?,
        align_content: align_content.parse(steps, parse_align_content)?,
        justify_content: justify_content.parse(steps, parse_align_content)?,
        gap: gap.parse(steps, |(w, h)| {
            crate::Result::Ok((parse_len(w)?, parse_len(h)?))
        })?,
        grid_template_rows: grid_template_rows.parse(steps, parse_grid_template)?,
        grid_template_columns: grid_template_columns.parse(steps, parse_grid_template)?,
        grid_row: grid_row.parse(steps, parse_grid_position)?,
        grid_column: grid_column.parse(steps, parse_grid_position)?,
        p_top: p_top.parse(steps, parse_len)?,
        p_bottom: p_bottom.parse(steps, parse_len)?,
        p_left: p_left.parse(steps, parse_len)?,
        p_right: p_right.parse(steps, parse_len)?,
        m_top: m_top.parse(steps, parse_len_auto)?,
        m_bottom: m_bottom.parse(steps, parse_len_auto)?,
        m_left: m_left.parse(steps, parse_len_auto)?,
        m_right: m_right.parse(steps, parse_len_auto)?,
        bg_color,
        content,
        url: url.into_step_value(steps),
        debug_layout: debug_layout.as_deref().map(Color::from_str).transpose()?,
        children: Vec::new(),
        styles,
    };
    Ok(node)
}
