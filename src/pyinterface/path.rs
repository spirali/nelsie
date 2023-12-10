use crate::common::error::NelsieError;
use crate::model::PathPart::{Cubic, Line, Move, Quad};
use crate::model::{Path, PathPart, SlideDeck, Stroke};
use crate::parsers::{parse_position, StringOrFloatOrExpr};
use crate::pyinterface::basictypes::PyStringOrFloatOrExpr;
use crate::pyinterface::deck::Deck;
use crate::pyinterface::resources::Resources;
use itertools::Itertools;
use pyo3::exceptions::PyException;
use pyo3::types::PyString;
use pyo3::{pyclass, pymethods, FromPyObject, PyAny, PyResult};
use taffy::prelude::points;

#[derive(Debug, FromPyObject)]
pub(crate) struct PyPath {
    stroke: Option<Stroke>,
    commands: Vec<String>,
    points: Vec<PyStringOrFloatOrExpr>,
}

fn command_to_part(
    command: &str,
    points_iter: &mut impl Iterator<Item = PyStringOrFloatOrExpr>,
) -> crate::Result<PathPart> {
    let mut next = || -> crate::Result<StringOrFloatOrExpr> {
        Ok(points_iter
            .next()
            .ok_or_else(|| NelsieError::generic_err("Point stack depleted"))?
            .into())
    };
    match command {
        "move" => Ok(Move {
            x: parse_position(&next()?, true)?,
            y: parse_position(&next()?, false)?,
        }),
        "line" => Ok(Line {
            x: parse_position(&next()?, true)?,
            y: parse_position(&next()?, false)?,
        }),
        "quad" => Ok(Quad {
            x1: parse_position(&next()?, true)?,
            y1: parse_position(&next()?, false)?,
            x: parse_position(&next()?, true)?,
            y: parse_position(&next()?, false)?,
        }),
        "cubic" => Ok(Cubic {
            x1: parse_position(&next()?, true)?,
            y1: parse_position(&next()?, false)?,
            x2: parse_position(&next()?, true)?,
            y2: parse_position(&next()?, false)?,
            x: parse_position(&next()?, true)?,
            y: parse_position(&next()?, false)?,
        }),
        _ => Err(NelsieError::generic_err("Invalid path command")),
    }
}

impl PyPath {
    pub fn into_path(self) -> crate::Result<Path> {
        let mut points_iter = self.points.into_iter();
        Ok(Path {
            stroke: self.stroke,
            parts: self
                .commands
                .into_iter()
                .map(|cmd| command_to_part(cmd.as_str(), &mut points_iter))
                .try_collect()?,
        })
    }
}

impl<'py> FromPyObject<'py> for Stroke {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        Ok(Stroke {
            color: ob.getattr("color")?.extract()?,
            width: ob.getattr("width")?.extract()?,
            dash_array: ob.getattr("dash_array")?.extract()?,
            dash_offset: ob.getattr("dash_offset")?.extract()?,
        })
    }
}
