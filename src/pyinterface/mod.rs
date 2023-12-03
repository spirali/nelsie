mod r#box;
mod deck;
mod error;
mod insteps;
mod resources;
mod textstyle;

use crate::pyinterface::insteps::ValueOrInSteps;
use crate::pyinterface::resources::Resources;
use deck::Deck;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

impl From<crate::NelsieError> for PyErr {
    fn from(err: crate::NelsieError) -> PyErr {
        PyException::new_err(err.to_string())
    }
}

// #[derive(Debug, FromPyObject)]
// enum StringOrInt {
//     Int(u32),
//     String(String),
// }
//
// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn test_abc(a: ValueOrInSteps<StringOrInt>) -> PyResult<String> {
//     Ok(format!("{:?}", a))
// }

/// A Python module implemented in Rust.
#[pymodule]
fn nelsie(_py: Python, m: &PyModule) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(test_abc, m)?)?;
    m.add_class::<Deck>()?;
    m.add_class::<Resources>()?;
    Ok(())
}
