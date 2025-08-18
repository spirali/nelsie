use crate::parsers::length::parse_string_length;
use crate::pyinterface::common::PyColor;
use crate::pyinterface::image::{LoadedImage, PyImage, PyImageData, PyImageFormat};
use crate::pyinterface::layoutexpr::extract_layout_expr;
use crate::pyinterface::shapes::{DimX, DimY, PyPath, PyPosition, PyRect};
use crate::pyinterface::text::PyTextContent;
use pyo3::conversion::FromPyObjectBound;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PyList;
use pyo3::{intern, Bound, FromPyObject, PyAny, PyResult};
use renderer::{
    Color, LayoutExpr, Length, LengthOrAuto, LengthOrExpr, Node, NodeChild, NodeId, Page,
    Rectangle, Register, Resources, Text,
};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

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
            Length::Points { value: value }
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
}

fn get<'a, 'py, T1: FromPyObjectBound<'a, 'py>, T2, F: FnOnce(T1) -> PyResult<T2>>(
    obj: &'a Bound<'py, PyAny>,
    name: &str,
    class: &str,
) -> PyResult<Bound<'py, PyAny>> {
    obj.getattr(name).map_err(|_| {
        PyValueError::new_err(format!(
            "Cannot found attribute '{name}' when extracting class '{class}'."
        ))
    })
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
                        .map(|f| check_font_or_fail(&f, resources))
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
                    }
                }
            })
        })
        .transpose()?
        .flatten();

    let i_node_id = intern!(obj.py(), "node_id");
    let i_shape = intern!(obj.py(), "shape");

    Ok(Node {
        node_id: NodeId::new(node.node_id),
        width: node.width.map(|x| x.0),
        height: node.height.map(|x| x.0),
        show: node.show,
        x: node.x.map(|x| x.expr),
        y: node.y.map(|x| x.expr),
        border_radius: 0.0,
        row: node.row,
        reverse: node.reverse,
        flex_wrap: Default::default(),
        flex_grow: 0.0,
        flex_shrink: 0.0,
        align_items: None,
        align_self: None,
        justify_self: None,
        align_content: None,
        justify_content: None,
        column_gap: Default::default(),
        row_gap: Default::default(),
        grid_template_rows: vec![],
        grid_template_columns: vec![],
        grid_row: Default::default(),
        grid_column: Default::default(),
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
        url: None,
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
