use crate::parsers::length::parse_string_length;
use pyo3::conversion::FromPyObjectBound;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PyList;
use pyo3::{Bound, FromPyObject, PyAny, PyResult};
use renderer::{Color, Length, LengthOrExpr, Node, NodeChild, NodeId, Page};

struct PyColor(Color);

impl<'py> FromPyObject<'py> for PyColor {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        let s: &str = obj.extract()?;
        Ok(PyColor(Color::from_str(s).map_err(crate::Error::from)?))
    }
}

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
        if let Ok(value) = obj.extract::<f32>() {
            return Ok(PyLengthOrExpr(LengthOrExpr::points(value)));
        }
        if let Ok(value) = obj.extract::<&str>() {
            return Ok(PyLengthOrExpr(LengthOrExpr::Length(parse_string_length(
                value,
            )?)));
        }
        todo!()
    }
}

#[derive(FromPyObject)]
struct PyNode<'py> {
    width: Option<PyLengthOrExpr>,
    height: Option<PyLengthOrExpr>,
    bg_color: Option<PyColor>,
    children: Bound<'py, PyList>,
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

fn obj_to_node(obj: Bound<PyAny>, id_counter: &mut NodeId) -> PyResult<Node> {
    let node_id = id_counter.bump();
    let node: PyNode = obj.extract()?;
    Ok(Node {
        node_id,
        width: node.width.map(|x| x.0),
        height: node.height.map(|x| x.0),
        show: true,
        x: None,
        y: None,
        border_radius: 0.0,
        row: false,
        reverse: false,
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
        p_top: Default::default(),
        p_bottom: Default::default(),
        p_left: Default::default(),
        p_right: Default::default(),
        m_top: Default::default(),
        m_bottom: Default::default(),
        m_left: Default::default(),
        m_right: Default::default(),
        bg_color: node.bg_color.map(|x| x.0),
        z_level: 0,
        content: Default::default(),
        url: None,
        children: node
            .children
            .try_iter()?
            .map(|child| {
                let child = child?;
                Ok(NodeChild::Node(obj_to_node(child, id_counter)?))
            })
            .collect::<PyResult<Vec<NodeChild>>>()?,
    })
}

pub fn obj_to_page(obj: Bound<PyAny>) -> PyResult<Page> {
    let py_page: PyPage = obj.extract()?;
    let mut id_counter = NodeId::new(0);
    Ok(Page::new(
        obj_to_node(py_page.root, &mut id_counter)?,
        py_page.width,
        py_page.height,
        py_page.bg_color.0,
    ))
}
