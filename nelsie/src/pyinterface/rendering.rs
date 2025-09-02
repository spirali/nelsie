use crate::pyinterface::extract::obj_to_page;
use crate::pyinterface::resources::Resources;
use pyo3::exceptions::PyException;
use pyo3::types::{PyDict, PyDictMethods, PyList};
use pyo3::{pyfunction, Bound, IntoPyObjectExt, PyAny, PyResult, Python};
use renderer::{Document, PageLayout, Register, RenderingOptions};
use std::collections::HashMap;

/// Formats the sum of two numbers as string.
#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub(crate) fn render<'py>(
    py: Python<'py>,
    resources: &mut Resources,
    pages: &Bound<'py, PyList>,
    path: Option<&'py str>,
    format: &str,
    compression_level: u8,
    n_threads: Option<usize>,
    progressbar: bool,
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
        progressbar,
    };
    let result = py.detach(|| run_rendering(resources, &options, path, format, doc))?;
    Ok(match result {
        RenderingOutput::None => py.None().into_bound(py),
        RenderingOutput::LayoutInfo(info) => {
            let v = info
                .into_iter()
                .map(|info| {
                    info.node_layouts
                        .into_iter()
                        .map(|(node_id, rect)| {
                            let rect_dict = PyDict::new(py);
                            rect_dict.set_item("x", rect.x)?;
                            rect_dict.set_item("y", rect.y)?;
                            rect_dict.set_item("width", rect.width)?;
                            rect_dict.set_item("height", rect.height)?;
                            Ok((node_id.as_usize(), rect_dict))
                        })
                        .collect::<PyResult<HashMap<_, _>>>()
                })
                .collect::<PyResult<Vec<_>>>()?;
            v.into_bound_py_any(py)?
        }
        RenderingOutput::SingleBinOutput(output) => output.into_bound_py_any(py)?,
        RenderingOutput::ManyBinOutputs(outputs) => outputs.into_bound_py_any(py)?,
        RenderingOutput::ManyStringOutputs(outputs) => outputs.into_bound_py_any(py)?,
    })
}

enum RenderingOutput {
    None,
    LayoutInfo(Vec<PageLayout>),
    SingleBinOutput(Vec<u8>),
    ManyBinOutputs(Vec<Vec<u8>>),
    ManyStringOutputs(Vec<String>),
}

fn run_rendering(
    resources: &Resources,
    options: &RenderingOptions,
    path: Option<&str>,
    format: &str,
    doc: Document,
) -> PyResult<RenderingOutput> {
    Ok(match (path, format) {
        (Some(path), "pdf") => {
            doc.render_pdf_to_file(&resources.resources, options, std::path::Path::new(path))
                .map_err(crate::Error::from)?;
            RenderingOutput::None
        }
        (Some(path), "png") => {
            doc.render_png_to_dir(&resources.resources, options, std::path::Path::new(path))
                .map_err(crate::Error::from)?;
            RenderingOutput::None
        }
        (Some(path), "svg") => {
            doc.render_svg_to_dir(&resources.resources, options, std::path::Path::new(path))
                .map_err(crate::Error::from)?;
            RenderingOutput::None
        }
        (_, "layout") => {
            let layout = doc
                .render_layout_info(&resources.resources, options)
                .map_err(crate::Error::from)?;
            RenderingOutput::LayoutInfo(layout)
        }
        (None, "pdf") => {
            let output = doc
                .render_pdf_to_mem(&resources.resources, options)
                .map_err(crate::Error::from)?;
            RenderingOutput::SingleBinOutput(output)
        }
        (None, "svg") => {
            let output = doc
                .render_svg_to_vec(&resources.resources, options)
                .map_err(crate::Error::from)?;
            RenderingOutput::ManyStringOutputs(output)
        }
        (None, "png") => {
            let output = doc
                .render_png_to_vec(&resources.resources, options)
                .map_err(crate::Error::from)?;
            RenderingOutput::ManyBinOutputs(output)
        }

        // (None, "png") => {
        //     doc.render_png_to_dir(&resources.resources, options, std::path::Path::new(path))
        //         .map_err(crate::Error::from)?;
        //     RenderingOutput::None
        // }
        // (None, "svg") => {
        //     doc.render_svg_to_dir(&resources.resources, options, std::path::Path::new(path))
        //         .map_err(crate::Error::from)?;
        //     RenderingOutput::None
        // }
        _ => {
            return Err(PyException::new_err(format!(
                "Invalid output format: {format}"
            )))
        }
    })
}
