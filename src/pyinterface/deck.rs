use crate::common::Step;
use crate::model::{Color, Drawing, Node, NodeChild, Path, Slide, SlideDeck, StepValue};
use crate::pyinterface::insteps::ValueOrInSteps;
use crate::pyinterface::path::PyPath;
use crate::pyinterface::r#box::{make_node, Content, NodeCreationEnv, Show};
use crate::pyinterface::resources::Resources;
use crate::pyinterface::textstyle::{partial_text_style_to_pyobject, PyTextStyle};
use crate::render::{render_slide_deck, OutputConfig, OutputFormat, VerboseLevel};
use itertools::Itertools;
use pyo3::exceptions::{PyException, PyValueError};
use pyo3::{pyclass, pymethods, PyObject, PyResult, Python, ToPyObject};
use std::collections::BTreeMap;
use std::str::FromStr;

use crate::pyinterface::basictypes::{PyStringOrFloat, PyStringOrFloatOrExpr};
use pyo3::types::{PyBytes, PyNone};
use std::sync::Arc;

#[pyclass]
pub(crate) struct Deck {
    deck: SlideDeck,
}

type SlideId = u32;
type BoxId = Vec<u32>;

fn resolve_box_id<'a>(node: &'a mut Node, box_id: &[u32]) -> PyResult<&'a mut Node> {
    if box_id.is_empty() {
        return Ok(node);
    }
    node.child_node_mut(box_id[0] as usize)
        .ok_or_else(|| PyException::new_err("Invalid box id"))
        .and_then(|child| resolve_box_id(child, &box_id[1..]))
}

fn resolve_slide_id(deck: &mut SlideDeck, slide_id: SlideId) -> PyResult<&mut Slide> {
    deck.slides
        .get_mut(slide_id as usize)
        .ok_or_else(|| PyException::new_err("Invalid slide id"))
}

#[pymethods]
impl Deck {
    #[new]
    fn new(
        resources: &mut Resources,
        default_font: Option<&str>,
        default_monospace_font: Option<&str>,
    ) -> PyResult<Self> {
        Ok(Deck {
            deck: SlideDeck::new(
                &mut resources.resources,
                default_font,
                default_monospace_font,
            )?,
        })
    }

    fn new_slide(
        &mut self,
        width: f32,
        height: f32,
        bg_color: &str,
        name: String,
        counters: Option<Vec<String>>,
    ) -> PyResult<SlideId> {
        let slide_id = self.deck.slides.len() as SlideId;
        self.deck.slides.push(Slide::new(
            width,
            height,
            name,
            Color::from_str(bg_color)?,
            self.deck.global_styles.clone(),
            counters.unwrap_or_default(),
        ));
        Ok(slide_id)
    }

    fn draw(
        &mut self,
        slide_id: SlideId,
        box_id: BoxId,
        paths: ValueOrInSteps<Vec<PyPath>>,
    ) -> PyResult<()> {
        let slide = resolve_slide_id(&mut self.deck, slide_id)?;
        let node = resolve_box_id(&mut slide.node, &box_id)?;
        let paths: StepValue<Vec<Path>> = paths.parse(&mut slide.n_steps, |paths| {
            paths.into_iter().map(|p| p.into_path()).try_collect()
        })?;
        node.children.push(NodeChild::Draw(Drawing { paths }));
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn new_box(
        &mut self,
        resources: &mut Resources,
        slide_id: SlideId,
        box_id: BoxId,

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
        replace_steps: Option<BTreeMap<crate::model::Step, crate::model::Step>>,
        content: Option<Content>,
    ) -> PyResult<(BoxId, u32)> {
        let slide = resolve_slide_id(&mut self.deck, slide_id)?;
        let node_id = slide.new_node_id();
        let parent_node = resolve_box_id(&mut slide.node, &box_id)?;

        let mut nce = NodeCreationEnv {
            resources: &mut resources.resources,
        };
        let (node, n_steps) = make_node(
            node_id,
            &mut nce,
            parent_node.styles.clone(),
            active,
            show,
            bg_color,
            x,
            y,
            width,
            height,
            border_radius,
            row,
            reverse,
            flex_wrap,
            flex_grow,
            flex_shrink,
            align_items,
            align_self,
            justify_self,
            align_content,
            justify_content,
            gap,
            p_left,
            p_right,
            p_top,
            p_bottom,
            m_left,
            m_right,
            m_top,
            m_bottom,
            z_level,
            name,
            debug_layout,
            replace_steps,
            content,
        )?;
        slide.n_steps = slide.n_steps.max(n_steps);

        let new_id = parent_node.children.len() as u32;
        let node_id = node.node_id;
        parent_node.children.push(NodeChild::Node(node));

        let mut new_box_id = box_id;
        new_box_id.push(new_id);
        Ok((new_box_id, node_id.as_u32()))
    }

    fn set_style(
        &mut self,
        resources: &mut Resources,
        name: &str,
        text_style: ValueOrInSteps<PyTextStyle>,
        update: bool,
        slide_id: Option<SlideId>,
        box_id: Option<BoxId>,
    ) -> PyResult<()> {
        let (styles, text_style) = if let Some(slide_id) = slide_id {
            let slide = resolve_slide_id(&mut self.deck, slide_id)?;
            if let Some(box_id) = box_id {
                let node = resolve_box_id(&mut slide.node, &box_id)?;
                let text_style = text_style.parse(&mut slide.n_steps, |s| {
                    s.into_partial_style(&resources.resources)
                })?;
                (&mut node.styles, text_style)
            } else {
                return Ok(());
            }
        } else {
            let text_style =
                text_style.parse_ignore_n_steps(|s| s.into_partial_style(&resources.resources))?;
            (&mut self.deck.global_styles, text_style)
        };
        let styles = Arc::make_mut(styles);
        if update {
            styles.update_style(name.to_string(), text_style);
        } else {
            styles.set_style(name.to_string(), text_style);
        }

        Ok(())
    }

    fn get_style(
        &mut self,
        py: Python<'_>,
        name: &str,
        step: Step,
        slide_id: Option<SlideId>,
        box_id: Option<BoxId>,
    ) -> PyResult<PyObject> {
        Ok((if let Some(slide_id) = slide_id {
            let slide = resolve_slide_id(&mut self.deck, slide_id)?;
            if let Some(box_id) = box_id {
                let node = resolve_box_id(&mut slide.node, &box_id)?;
                node.styles
                    .get_style(name)
                    .map(|style| partial_text_style_to_pyobject(style.at_step(step), py))?
            } else {
                return Err(PyException::new_err("Invalid box id"));
            }
        } else {
            self.deck
                .global_styles
                .get_style(name)
                .map(|style| partial_text_style_to_pyobject(style.at_step(step), py))?
        })
        .to_object(py))
    }

    fn set_n_steps(&mut self, slide_id: SlideId, value: u32) -> PyResult<()> {
        resolve_slide_id(&mut self.deck, slide_id)?.n_steps = value.max(1);
        Ok(())
    }

    fn get_n_steps(&mut self, slide_id: SlideId) -> PyResult<u32> {
        Ok(resolve_slide_id(&mut self.deck, slide_id)?.n_steps)
    }

    fn render(
        &self,
        py: Python<'_>,
        resources: &mut Resources,
        verbose: u32,
        format: &str,
        path: Option<&str>,
    ) -> PyResult<PyObject> {
        let verbose_level = match verbose {
            0 => VerboseLevel::Silent,
            1 => VerboseLevel::Normal,
            2 => VerboseLevel::Full,
            _ => return Err(PyValueError::new_err("Invalid verbose level")),
        };
        let format = match format {
            "pdf" => OutputFormat::Pdf,
            "svg" => OutputFormat::Svg,
            "png" => OutputFormat::Png,
            _ => return Err(PyValueError::new_err("Unknown output format")),
        };
        let result = py.allow_threads(|| {
            render_slide_deck(
                &self.deck,
                &resources.resources,
                &OutputConfig {
                    path: path.map(std::path::Path::new),
                    format,
                },
                verbose_level,
            )
        })?;
        if result.is_empty() {
            Ok(PyNone::get(py).to_object(py))
        } else {
            Ok(result
                .iter()
                .map(|(slide_idx, step, data)| (slide_idx, step, PyBytes::new(py, data)))
                .collect_vec()
                .to_object(py))
        }
    }
}
