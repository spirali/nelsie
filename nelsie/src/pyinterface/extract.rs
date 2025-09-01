use crate::parsers::length::parse_string_length;
use crate::pyinterface::common::PyColor;
use crate::pyinterface::image::{PyImage, PyImageData};
use crate::pyinterface::layoutexpr::extract_layout_expr;
use crate::pyinterface::shapes::{DimX, DimY, PyPath, PyPosition, PyRect};
use crate::pyinterface::text::PyTextContent;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PyList;
use pyo3::{intern, Bound, FromPyObject, PyAny, PyResult};
use renderer::taffy::style_helpers::{
    FromFlex, FromLength, FromPercent, TaffyGridLine, TaffyGridSpan,
};
use renderer::taffy::{
    AlignContent, AlignItems, GridPlacement, Line, NonRepeatedTrackSizingFunction,
};
use renderer::{
    Length, LengthOrAuto, LengthOrExpr, Node, NodeChild, NodeId, Page, Rectangle, Register,
    Resources, Text,
};

#[derive(FromPyObject)]
struct PyPage<'py> {
    width: f32,
    height: f32,
    bg_color: PyColor,
    root: Bound<'py, PyAny>,
}

struct PyLengthOrExpr(LengthOrExpr);

impl<'py> FromPyObject<'py> for PyLengthOrExpr {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(PyLengthOrExpr(if let Ok(value) = obj.extract::<f32>() {
            LengthOrExpr::points(value)
        } else if let Ok(value) = obj.extract::<&str>() {
            LengthOrExpr::Length(parse_string_length(value)?)
        } else {
            LengthOrExpr::Expr(extract_layout_expr(obj)?)
        }))
    }
}

struct PyLength(Length);

impl<'py> FromPyObject<'py> for PyLength {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(PyLength(if let Ok(value) = obj.extract::<f32>() {
            Length::Points { value }
        } else if let Ok(value) = obj.extract::<&str>() {
            parse_string_length(value)?
        } else {
            return Err(PyValueError::new_err("Invalid length definition"));
        }))
    }
}

struct PyLengthOrAuto(LengthOrAuto);

impl<'py> FromPyObject<'py> for PyLengthOrAuto {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(PyLengthOrAuto(if let Ok(value) = obj.extract::<f32>() {
            LengthOrAuto::Length(Length::Points { value })
        } else if let Ok(value) = obj.extract::<&str>() {
            LengthOrAuto::Length(parse_string_length(value)?)
        } else {
            return Err(PyValueError::new_err("Invalid length definition"));
        }))
    }
}

struct PyAlignItems(AlignItems);

impl<'py> FromPyObject<'py> for PyAlignItems {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        let s = obj.extract::<&str>()?;
        Ok(PyAlignItems(match s {
            "start" => AlignItems::Start,
            "end" => AlignItems::End,
            "flex-start" => AlignItems::FlexStart,
            "flex-end" => AlignItems::FlexEnd,
            "center" => AlignItems::Center,
            "stretch" => AlignItems::Stretch,
            "baseline" => AlignItems::Baseline,
            _ => return Err(PyValueError::new_err("Invalid AlignItems")),
        }))
    }
}

impl From<PyAlignItems> for AlignItems {
    fn from(value: PyAlignItems) -> Self {
        value.0
    }
}

struct PyAlignContent(AlignContent);

impl<'py> FromPyObject<'py> for PyAlignContent {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        let s = obj.extract::<&str>()?;
        Ok(PyAlignContent(match s {
            "start" => AlignContent::Start,
            "end" => AlignContent::End,
            "flex-start" => AlignContent::FlexStart,
            "flex-end" => AlignContent::FlexEnd,
            "center" => AlignContent::Center,
            "stretch" => AlignContent::Stretch,
            "space-between" => AlignContent::SpaceBetween,
            "space-evenly" => AlignContent::SpaceEvenly,
            "space-around" => AlignContent::SpaceAround,
            _ => return Err(PyValueError::new_err("Invalid AlignContent")),
        }))
    }
}

impl From<PyAlignContent> for AlignContent {
    fn from(value: PyAlignContent) -> Self {
        value.0
    }
}

struct PyGridTemplateItem(NonRepeatedTrackSizingFunction);

impl<'py> FromPyObject<'py> for PyGridTemplateItem {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(PyGridTemplateItem(
            if let Ok(value) = obj.extract::<f32>() {
                NonRepeatedTrackSizingFunction::from_length(value)
            } else if let Ok(value) = obj.extract::<&str>() {
                let value = value.trim();
                if let Some(value) = value.strip_suffix("%") {
                    let value = value.trim();
                    NonRepeatedTrackSizingFunction::from_percent(value.parse::<f32>()? / 100.0)
                } else if let Some(value) = value.strip_suffix("fr") {
                    let value = value.trim();
                    NonRepeatedTrackSizingFunction::from_flex(value.parse::<f32>()?)
                } else if let Ok(value) = value.parse::<f32>() {
                    NonRepeatedTrackSizingFunction::from_length(value)
                } else {
                    return Err(PyValueError::new_err(format!(
                        "Invalid grid template: {value}"
                    )));
                }
            } else {
                return Err(PyValueError::new_err("Invalid grid template"));
            },
        ))
    }
}

impl From<PyGridTemplateItem> for NonRepeatedTrackSizingFunction {
    fn from(value: PyGridTemplateItem) -> Self {
        value.0
    }
}

struct PyGridLinePlacement(Line<GridPlacement>);

fn parse_grid_placement_item(obj: &Bound<PyAny>) -> PyResult<GridPlacement> {
    if obj.is_none() {
        return Ok(GridPlacement::Auto);
    }
    if let Ok(value) = obj.extract::<&str>() {
        let value = value.trim();
        if value == "auto" {
            return Ok(GridPlacement::Auto);
        }
        if let Some(value) = value.strip_prefix("span ") {
            let value: u16 = value.trim().parse()?;
            return Ok(GridPlacement::from_span(value));
        }
        if let Ok(value) = value.parse() {
            return Ok(GridPlacement::from_line_index(value));
        }
        Err(PyValueError::new_err("Invalid grid placement"))
    } else if let Ok(value) = obj.extract::<i16>() {
        Ok(GridPlacement::from_line_index(value))
    } else {
        Err(PyValueError::new_err("Invalid grid placement"))
    }
}

impl<'py> FromPyObject<'py> for PyGridLinePlacement {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(value) = parse_grid_placement_item(obj) {
            return Ok(PyGridLinePlacement(Line {
                start: value,
                end: GridPlacement::Auto,
            }));
        } else if let Ok((value1, value2)) = obj.extract::<(Bound<'py, PyAny>, Bound<'py, PyAny>)>()
        {
            if let Ok(start) = parse_grid_placement_item(&value1) {
                if let Ok(end) = parse_grid_placement_item(&value2) {
                    return Ok(PyGridLinePlacement(Line { start, end }));
                }
            }
        }
        Err(PyValueError::new_err("Invalid grid placement"))
    }
}

#[derive(FromPyObject)]
struct PyGridOptions {
    template_rows: Vec<PyGridTemplateItem>,
    template_columns: Vec<PyGridTemplateItem>,
    row: PyGridLinePlacement,
    column: PyGridLinePlacement,
}

#[derive(FromPyObject)]
enum NodeContent<'py> {
    Text(PyTextContent),
    Image(Bound<'py, PyImage>),
}

#[derive(FromPyObject)]
struct PyNode<'py> {
    node_id: usize,
    x: Option<PyPosition<DimX>>,
    y: Option<PyPosition<DimY>>,
    show: bool,
    z_level: i32,
    width: Option<PyLengthOrExpr>,
    height: Option<PyLengthOrExpr>,
    bg_color: Option<PyColor>,
    border_radius: f32,
    row: bool,
    reverse: bool,
    children: Bound<'py, PyList>,
    content: Option<NodeContent<'py>>,
    p_left: PyLength,
    p_right: PyLength,
    p_top: PyLength,
    p_bottom: PyLength,
    m_left: PyLengthOrAuto,
    m_right: PyLengthOrAuto,
    m_top: PyLengthOrAuto,
    m_bottom: PyLengthOrAuto,
    flex_grow: f32,
    flex_shrink: f32,
    align_items: Option<PyAlignItems>,
    align_self: Option<PyAlignItems>,
    justify_self: Option<PyAlignItems>,
    align_content: Option<PyAlignContent>,
    justify_content: Option<PyAlignContent>,
    gap_x: PyLength,
    gap_y: PyLength,
    grid: Option<PyGridOptions>,
    url: Option<String>,
}

fn check_font_or_fail(font: &str, resources: &mut Resources) -> PyResult<()> {
    if !resources.check_font(font) {
        return Err(PyValueError::new_err(format!("Font '{font}' not found.")));
    }
    Ok(())
}

fn obj_to_node(
    obj: Bound<PyAny>,
    register: &mut Register,
    resources: &mut Resources,
) -> PyResult<Node> {
    let node: PyNode = obj.extract()?;
    let content = node
        .content
        .map(|content| -> PyResult<_> {
            Ok(match content {
                NodeContent::Text(text) => {
                    let text: Text = text.try_into()?;
                    text.style
                        .font
                        .as_ref()
                        .map(|f| check_font_or_fail(f, resources))
                        .transpose()?;
                    Some(register.register_text(text))
                }
                NodeContent::Image(image) => {
                    let image = image.get();
                    match &image.image_data {
                        PyImageData::BinImage(img) => Some(register.register_bin_image(
                            img.clone(),
                            image.width,
                            image.height,
                        )),
                        PyImageData::SvgImage(img) => Some(register.register_svg_image(
                            img.clone(),
                            image.width,
                            image.height,
                        )),
                        PyImageData::FragmentedSvgImage(img) => {
                            let items: Vec<_> = img
                                .iter()
                                .map(|layer| {
                                    (
                                        Rectangle::new(0.0, 0.0, image.width, image.height),
                                        register.register_svg_image(
                                            layer.clone(),
                                            image.width,
                                            image.height,
                                        ),
                                    )
                                })
                                .collect();
                            if items.is_empty() {
                                None
                            } else if items.len() == 1 {
                                Some(items[0].1)
                            } else {
                                Some(register.register_composition(
                                    image.width,
                                    image.height,
                                    items,
                                ))
                            }
                        }
                        PyImageData::Ora(img) => {
                            let items: Vec<_> = img
                                .iter()
                                .map(|(rectangle, layer)| {
                                    (
                                        rectangle.clone(),
                                        register.register_bin_image(
                                            layer.clone(),
                                            rectangle.width,
                                            rectangle.height,
                                        ),
                                    )
                                })
                                .collect();
                            if items.is_empty() {
                                None
                            } else {
                                Some(register.register_composition(
                                    image.width,
                                    image.height,
                                    items,
                                ))
                            }
                        }
                    }
                }
            })
        })
        .transpose()?
        .flatten();

    let i_node_id = intern!(obj.py(), "node_id");
    let i_shape = intern!(obj.py(), "shape");

    let (grid_template_rows, grid_template_columns, grid_row, grid_column) =
        if let Some(o) = node.grid {
            (
                o.template_rows.into_iter().map(|x| x.into()).collect(),
                o.template_columns.into_iter().map(|x| x.into()).collect(),
                o.row.0,
                o.column.0,
            )
        } else {
            (
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            )
        };

    Ok(Node {
        grid_template_rows,
        grid_template_columns,
        grid_row,
        grid_column,
        node_id: NodeId::new(node.node_id),
        width: node.width.map(|x| x.0),
        height: node.height.map(|x| x.0),
        show: node.show,
        x: node.x.map(|x| x.expr),
        y: node.y.map(|x| x.expr),
        border_radius: node.border_radius,
        row: node.row,
        reverse: node.reverse,
        flex_wrap: Default::default(),
        flex_grow: node.flex_grow,
        flex_shrink: node.flex_shrink,
        align_items: node.align_items.map(|x| x.into()),
        align_self: node.align_self.map(|x| x.into()),
        justify_self: node.justify_self.map(|x| x.into()),
        align_content: node.align_content.map(|x| x.into()),
        justify_content: node.justify_content.map(|x| x.into()),
        column_gap: node.gap_x.0,
        row_gap: node.gap_y.0,
        p_top: node.p_top.0,
        p_bottom: node.p_bottom.0,
        p_left: node.p_left.0,
        p_right: node.p_right.0,
        m_top: node.m_top.0,
        m_bottom: node.m_bottom.0,
        m_left: node.m_left.0,
        m_right: node.m_right.0,
        bg_color: node.bg_color.map(|x| x.into()),
        z_level: node.z_level,
        content,
        url: node.url,
        children: node
            .children
            .try_iter()?
            .map(|child| {
                let child = child?;
                Ok(if child.hasattr(i_node_id)? {
                    NodeChild::Node(obj_to_node(child, register, resources)?)
                } else if child.hasattr(i_shape)? {
                    let rect: PyRect = child.extract()?;
                    NodeChild::Shape(rect.into_shape())
                } else {
                    let path: PyPath = child.extract()?;
                    NodeChild::Shape(path.into_shape()?)
                })
            })
            .collect::<PyResult<Vec<NodeChild>>>()?,
    })
}

pub fn obj_to_page(
    obj: Bound<PyAny>,
    register: &mut Register,
    resources: &mut Resources,
) -> PyResult<Page> {
    let py_page: PyPage = obj.extract()?;
    Ok(Page::new(
        obj_to_node(py_page.root, register, resources)?,
        py_page.width,
        py_page.height,
        py_page.bg_color.into(),
    ))
}
