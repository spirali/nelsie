use crate::parsers::length::parse_string_length;
use crate::pyinterface::extract::obj_to_page;
use crate::pyinterface::resources::Resources;
use pyo3::conversion::FromPyObjectBound;
use pyo3::exceptions::{PyException, PyValueError};
use pyo3::types::{PyAnyMethods, PyDict, PyDictMethods, PyList, PyNone};
use pyo3::{pyfunction, Bound, FromPyObject, Py, PyAny, PyObject, PyResult, Python};
use renderer::{
    Color, Document, LengthOrExpr, Node, NodeChild, NodeId, Page, Register, RenderingOptions,
};
use std::collections::HashMap;
use std::sync::Arc;

/// Formats the sum of two numbers as string.
#[pyfunction]
pub(crate) fn render<'py>(
    py: Python<'py>,
    resources: &mut Resources,
    pages: &Bound<'py, PyList>,
    path: Option<&'py str>,
    format: &str,
    compression_level: u8,
    n_threads: Option<usize>,
) -> PyResult<Bound<'py, PyAny>> {
    let mut register = Register::new();
    let pages: Vec<_> = pages
        .into_iter()
        .map(|obj| obj_to_page(obj, &mut register, &mut resources.resources))
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
        (Some(path), "svg") => {
            doc.render_svg_to_dir(&resources.resources, options, std::path::Path::new(path))
                .map_err(crate::Error::from)?;
        }
        _ => {
            println!("TODO RENDER");
            todo!()
        }
    }
    Ok(())
}
