use super::node::Node;
use crate::common::error::NelsieError;
use crate::model::textstyles::FontData;
use crate::model::{
    Color, Length, LengthOrAuto, LengthOrExpr, NodeId, PartialTextStyle, Resources, Step, StepSet,
    StepValue, StyleMap,
};
use std::collections::Bound::{Included, Unbounded};
use std::collections::HashMap;
use std::sync::Arc;
use svg2pdf::usvg;
use taffy::prelude as tf;
use taffy::style::FlexWrap;
use usvg::FontStretch;

pub(crate) type SlideId = u32;

#[derive(Debug)]
pub(crate) struct Slide {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) node: Node,
    pub(crate) steps: StepSet,
    pub(crate) bg_color: Color,
    pub(crate) debug_steps: bool,
    pub(crate) counters: Vec<String>,
    pub(crate) parent: Option<(SlideId, Step)>,
    node_id_counter: NodeId,
}

impl Slide {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        width: f32,
        height: f32,
        name: String,
        bg_color: Color,
        debug_steps: bool,
        styles: Arc<StyleMap>,
        counters: Vec<String>,
        parent: Option<(SlideId, Step)>,
        step_1: bool,
    ) -> Self {
        let mut steps = StepSet::new();
        if step_1 {
            steps.insert(Step::from_int(1));
        }
        Slide {
            width,
            height,
            bg_color,
            debug_steps,
            counters,
            parent,
            node: Node {
                styles,
                name,
                node_id: NodeId::new(0),
                children: vec![],
                replace_steps: Default::default(),
                active: StepValue::new_const(true),
                show: StepValue::new_const(true),
                z_level: StepValue::Const(0),
                x: StepValue::Const(None),
                y: StepValue::Const(None),
                width: StepValue::Const(Some(LengthOrExpr::Points { value: width })),
                height: StepValue::Const(Some(LengthOrExpr::Points { value: height })),
                border_radius: StepValue::Const(0.0),
                row: StepValue::Const(false),
                reverse: StepValue::Const(false),
                flex_wrap: StepValue::Const(FlexWrap::NoWrap),
                flex_grow: StepValue::Const(0.0),
                flex_shrink: StepValue::Const(1.0),
                justify_content: StepValue::Const(Some(tf::JustifyContent::Center)),
                align_items: StepValue::Const(Some(tf::AlignItems::Center)),
                align_self: StepValue::Const(None),
                justify_self: StepValue::Const(None),
                align_content: StepValue::Const(None),
                gap: StepValue::Const((Length::ZERO, Length::ZERO)),
                p_top: StepValue::Const(Length::ZERO),
                p_bottom: StepValue::Const(Length::ZERO),
                p_left: StepValue::Const(Length::ZERO),
                p_right: StepValue::Const(Length::ZERO),
                m_top: StepValue::Const(LengthOrAuto::ZERO),
                m_bottom: StepValue::Const(LengthOrAuto::ZERO),
                m_left: StepValue::Const(LengthOrAuto::ZERO),
                m_right: StepValue::Const(LengthOrAuto::ZERO),
                bg_color: StepValue::Const(None),
                content: None,
                url: StepValue::Const(None),
                debug_layout: None,
            },
            steps,
            node_id_counter: NodeId::new(0),
        }
    }
    pub fn new_node_id(&mut self) -> NodeId {
        self.node_id_counter.bump()
    }

    pub fn visible_steps(&self) -> impl Iterator<Item = &Step> {
        self.steps.range((Included(Step::from_int(1)), Unbounded))
    }
}

#[derive(Debug)]
pub(crate) struct SlideDeck {
    pub(crate) slides: Vec<Slide>,
    pub(crate) global_styles: Arc<StyleMap>,
    pub(crate) default_font: Arc<FontData>,
    pub(crate) creation_time: std::time::Instant,
}

fn detect_font(
    resources: &mut Resources,
    forced_name: Option<&str>,
    options: &[&'static str],
    err: &str,
) -> crate::Result<FontData> {
    Ok(if let Some(font) = forced_name {
        resources.check_font(font)?
    } else {
        options
            .iter()
            .find_map(|n| resources.check_font(n).ok())
            .ok_or_else(|| NelsieError::generic_err(err))?
    })
}

impl SlideDeck {
    pub fn new(
        resources: &mut Resources,
        default_font: Option<&str>,
        default_monospace_font: Option<&str>,
    ) -> crate::Result<Self> {
        let creation_time = std::time::Instant::now();
        let default_font = detect_font(
            resources,
            default_font,
            &["DejaVu Sans", "Arial"],
            "No default font detected. Specify parameter 'default_font' in SlideDeck",
        )?;
        let default_monospace_font_family =
            detect_font(
                resources,
                default_monospace_font,
                &["DejaVu Sans Mono", "Courier New", "Courier"],
                "No default monospace font detected. Specify parameter 'default_monospace_font' in SlideDeck",
            )?;

        let default_font = Arc::new(default_font);
        let default_style = PartialTextStyle {
            font: Some(default_font.clone()),
            stroke: Some(None),
            color: Some(Some(Color::new(svgtypes::Color::black()))),
            size: Some(32.0),
            line_spacing: Some(1.2),
            italic: Some(false),
            stretch: Some(FontStretch::Normal),
            weight: Some(400),
            underline: Some(false),
            overline: Some(false),
            line_through: Some(false),
        };
        let monospace_style = PartialTextStyle {
            font: Some(Arc::new(default_monospace_font_family)),
            ..Default::default()
        };
        let mut styles = HashMap::new();
        styles.insert("default".to_string(), StepValue::new_const(default_style));
        styles.insert(
            "monospace".to_string(),
            StepValue::new_const(monospace_style.clone()),
        );
        styles.insert("code".to_string(), StepValue::new_const(monospace_style));
        Ok(Self {
            slides: Vec::new(),
            global_styles: Arc::new(StyleMap::new(styles)),
            default_font,
            creation_time,
        })
    }
}
