use crate::pyinterface::resources::Resources;
use pyo3::exceptions::PyException;
use pyo3::types::{PyAnyMethods, PyList, PyNone};
use pyo3::{pyfunction, Bound, PyAny, PyObject, PyResult, Python};
use renderer::{Color, Node, Page};

/// Formats the sum of two numbers as string.
#[pyfunction]
pub(crate) fn render_pages<'py>(
    py: Python<'py>,
    resources: &Resources,
    pages: &Bound<'py, PyList>,
    format: &str,
    path: Option<&'py str>,
) -> PyResult<Bound<'py, PyAny>> {
    let pages: Vec<_> = pages
        .into_iter()
        .map(|obj| obj_to_page(obj))
        .collect::<PyResult<Vec<_>>>()?;
    Ok(py.None().into_bound(py))
}

fn str_to_color(s: &str) -> PyResult<Color> {
    Ok(Color::from_str(s).map_err(crate::Error::from)?)
}

fn obj_to_node(obj: Bound<PyAny>) -> PyResult<Node> {
    todo!()
}

fn obj_to_page(obj: Bound<PyAny>) -> PyResult<Page> {
    let width = obj.getattr("width")?.extract()?;
    let height = obj.getattr("height")?.extract()?;
    let color = str_to_color(obj.getattr("color")?.extract::<&str>()?)?;
    let root = obj_to_node(obj.getattr("root")?)?;
    Ok(Page::new(root, width, height, color))
}
