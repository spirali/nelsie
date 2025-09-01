mod check;
mod common;
mod extract;
mod image;
mod layoutexpr;
mod ora;
mod parsers;
mod rendering;
mod resources;
mod shapes;
mod text;
mod watch;

use crate::pyinterface::image::LoadedImage;
use crate::pyinterface::parsers::parse_bool_steps;
use crate::pyinterface::resources::Resources;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

#[pymodule]
fn nelsie(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Resources>()?;
    m.add_function(wrap_pyfunction!(check::check_color, m)?)?;
    m.add_function(wrap_pyfunction!(rendering::render, m)?)?;
    m.add_function(wrap_pyfunction!(image::create_mem_image, m)?)?;
    m.add_function(wrap_pyfunction!(image::load_image, m)?)?;
    m.add_function(wrap_pyfunction!(parse_bool_steps, m)?)?;
    m.add_function(wrap_pyfunction!(watch::watch, m)?)?;
    m.add_class::<LoadedImage>()?;
    Ok(())
}

impl From<crate::Error> for PyErr {
    fn from(err: crate::Error) -> PyErr {
        PyException::new_err(err.to_string())
    }
}
