use crate::common::error::NelsieError;
use crate::model::{Arrow, Color, Path, PathPart, Stroke};
use crate::parsers::{parse_position, StringOrFloatOrExpr};
use crate::pyinterface::basictypes::PyStringOrFloatOrExpr;

use itertools::Itertools;

use pyo3::{FromPyObject, PyAny, PyResult};

#[derive(Debug)]
pub(crate) struct PyArrow(Arrow);

impl<'py> FromPyObject<'py> for PyArrow {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        Ok(PyArrow(Arrow {
            size: ob.getattr("size")?.extract()?,
            angle: ob.getattr("angle")?.extract()?,
            color: ob.getattr("color")?.extract()?,
            stroke_width: ob.getattr("stroke_width")?.extract()?,
            inner_point: ob.getattr("inner_point")?.extract()?,
        }))
    }
}

impl From<PyArrow> for Arrow {
    fn from(value: PyArrow) -> Self {
        value.0
    }
}

#[derive(Debug, FromPyObject)]
pub(crate) struct PyPath {
    stroke: Option<Stroke>,
    fill_color: Option<Color>,
    commands: Vec<String>,
    points: Vec<PyStringOrFloatOrExpr>,
    arrow_start: Option<PyArrow>,
    arrow_end: Option<PyArrow>,
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
        "move" => Ok(PathPart::Move {
            x: parse_position(next()?, true)?,
            y: parse_position(next()?, false)?,
        }),
        "line" => Ok(PathPart::Line {
            x: parse_position(next()?, true)?,
            y: parse_position(next()?, false)?,
        }),
        "quad" => Ok(PathPart::Quad {
            x1: parse_position(next()?, true)?,
            y1: parse_position(next()?, false)?,
            x: parse_position(next()?, true)?,
            y: parse_position(next()?, false)?,
        }),
        "cubic" => Ok(PathPart::Cubic {
            x1: parse_position(next()?, true)?,
            y1: parse_position(next()?, false)?,
            x2: parse_position(next()?, true)?,
            y2: parse_position(next()?, false)?,
            x: parse_position(next()?, true)?,
            y: parse_position(next()?, false)?,
        }),
        "close" => Ok(PathPart::Close),
        "oval" => Ok(PathPart::Oval {
            x1: parse_position(next()?, true)?,
            y1: parse_position(next()?, false)?,
            x2: parse_position(next()?, true)?,
            y2: parse_position(next()?, false)?,
        }),
        _ => Err(NelsieError::generic_err("Invalid path command")),
    }
}

impl PyPath {
    pub fn into_path(self) -> crate::Result<Path> {
        let mut points_iter = self.points.into_iter();
        Ok(Path {
            stroke: self.stroke,
            fill_color: self.fill_color,
            parts: self
                .commands
                .into_iter()
                .map(|cmd| command_to_part(cmd.as_str(), &mut points_iter))
                .try_collect()?,
            arrow_start: self.arrow_start.map(|x| x.into()),
            arrow_end: self.arrow_end.map(|x| x.into()),
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
