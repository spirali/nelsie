mod basictypes;
mod r#box;
mod deck;
mod error;
mod insteps;
mod layoutexpr;
mod path;
mod resources;
mod textstyle;

use crate::pyinterface::resources::Resources;
use deck::Deck;
use pyo3::exceptions::PyException;
use pyo3::types::PyModule;
use pyo3::{pymodule, Bound, PyErr, PyResult, Python};

impl From<crate::NelsieError> for PyErr {
    fn from(err: crate::NelsieError) -> PyErr {
        PyException::new_err(err.to_string())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn nelsie(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Deck>()?;
    m.add_class::<Resources>()?;
    Ok(())
}
