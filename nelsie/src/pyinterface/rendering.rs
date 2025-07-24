use crate::parsers::length::parse_string_length;
use crate::pyinterface::resources::Resources;
use pyo3::conversion::FromPyObjectBound;
use pyo3::exceptions::{PyException, PyValueError};
use pyo3::types::{PyAnyMethods, PyList, PyNone};
use pyo3::{pyfunction, Bound, FromPyObject, Py, PyAny, PyObject, PyResult, Python};
use renderer::{
    Color, Document, LengthOrExpr, Node, NodeChild, NodeId, Page, Register, RenderingOptions,
};

/// Formats the sum of two numbers as string.
#[pyfunction]
pub(crate) fn render<'py>(
    py: Python<'py>,
    resources: &Resources,
    pages: &Bound<'py, PyList>,
    path: Option<&'py str>,
    format: &str,
    compression_level: u8,
    n_threads: Option<usize>,
) -> PyResult<Bound<'py, PyAny>> {
    let mut register = Register::new();
    let pages: Vec<_> = pages
        .into_iter()
        .map(|obj| obj_to_page(obj))
        .collect::<PyResult<Vec<_>>>()?;
    let doc = Document::new(pages, register);

    let options = RenderingOptions {
        compression_level,
        n_threads,
    };
    let result = py.allow_threads(|| run_rendering(resources, &options, path, format, doc))?;
    Ok(py.None().into_bound(py))
}

fn run_rendering(
    resources: &Resources,
    options: &RenderingOptions,
    path: Option<&str>,
    format: &str,
    doc: Document,
) -> PyResult<()> {
    match (path, format) {
        (Some(path), "pdf") => {
            doc.render_pdf_to_file(&resources.resources, options, std::path::Path::new(path))
                .map_err(crate::Error::from)?;
        }
        (Some(path), "png") => {
            doc.render_png_to_dir(&resources.resources, options, std::path::Path::new(path))
                .map_err(crate::Error::from)?;
        }
        _ => {
            println!("TODO RENDER");
            todo!()
        }
    }
    Ok(())
}

fn str_to_color(s: &str) -> PyResult<Color> {
    Ok(Color::from_str(s).map_err(crate::Error::from)?)
}

struct PyColor(Color);

impl<'py> FromPyObject<'py> for PyColor {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let s: &str = ob.extract()?;
        Ok(PyColor(str_to_color(s)?))
    }
}

#[derive(FromPyObject)]
struct PyPage<'py> {
    width: f32,
    height: f32,
    color: PyColor,
    root: Bound<'py, PyAny>,
}

macro_rules! xget {
    ($obj: expr, $name:expr, $class:expr) => {{
        $obj.getattr($name)
            .map_err(|_| {
                PyValueError::new_err(format!(
                    "Cannot found attribute '{}' when extracting class '{}'.",
                    $name, $class
                ))
            })?
            .extract()
            .map_err(|_| {
                PyValueError::new_err(format!(
                    "Cannot extract argument '{}' for class '{}'.",
                    $name, $class,
                ))
            })?
    }};
}

macro_rules! get {
    ($obj: expr, $name:expr, $class:expr) => {{
        $obj.getattr($name).map_err(|_| {
            PyValueError::new_err(format!(
                "Cannot found attribute '{}' when extracting class '{}'.",
                $name, $class
            ))
        })?
    }};
}

// fn get<'a, 'py, T: FromPyObjectBound<'a, 'py>>(
//     obj: &'a Bound<'py, PyAny>,
//     name: &str,
//     class: &str,
// ) -> PyResult<T> {
//     obj.getattr(name)
//         .map_err(|_| {
//             PyValueError::new_err(format!(
//                 "Cannot found attribute '{name}' when extracting class '{class}'."
//             ))
//         })?
//         .extract()
//         .map_err(|_| {
//             PyValueError::new_err(format!(
//                 "Cannot extract argument '{name}' for class '{class}'."
//             ))
//         })
// }

fn obj_to_size(obj: Bound<PyAny>) -> PyResult<Option<LengthOrExpr>> {
    if obj.is_none() {
        return Ok(None);
    }
    if let Ok(value) = obj.extract::<f32>() {
        return Ok(Some(LengthOrExpr::points(value)));
    }
    if let Ok(value) = obj.extract::<&str>() {
        return Ok(Some(LengthOrExpr::Length(parse_string_length(value)?)));
    }
    println!("TODO 2");
    todo!()
}

fn obj_to_node(obj: Bound<PyAny>, id_counter: &mut NodeId) -> PyResult<Node> {
    let node_id = id_counter.bump();
    let children: Bound<PyList> = xget!(obj, "children", "Page");
    Ok(Node {
        node_id,
        width: obj_to_size(get!(obj, "width", "Node"))?,
        height: obj_to_size(get!(obj, "height", "Node"))?,
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
        bg_color: {
            let v = get!(obj, "bg_color", "Node");
            if v.is_none() {
                None
            } else {
                // TODO: custom extract
                Some(str_to_color(v.extract()?)?)
            }
        },
        z_level: 0,
        content: Default::default(),
        url: None,
        children: children
            .try_iter()?
            .map(|child| {
                let child = child?;
                Ok(NodeChild::Node(obj_to_node(child, id_counter)?))
            })
            .collect::<PyResult<Vec<NodeChild>>>()?,
    })
}

fn obj_to_page(obj: Bound<PyAny>) -> PyResult<Page> {
    let width = xget!(obj, "width", "Page");
    let height = xget!(obj, "height", "Page");
    let color = str_to_color(xget!(obj, "bg_color", "Page"))?;
    let mut id_counter = NodeId::new(0);
    let root = obj_to_node(obj.getattr("root")?, &mut id_counter)?;
    Ok(Page::new(root, width, height, color))
}
