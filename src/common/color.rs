use crate::common::error::NelsieError;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Color(svgtypes::Color);

impl Color {
    pub fn new(color: svgtypes::Color) -> Self {
        Color(color)
    }

    pub fn as_3f32(&self) -> (f32, f32, f32) {
        (
            self.0.red as f32 / 255.0,
            self.0.green as f32 / 255.0,
            self.0.blue as f32 / 255.0,
        )
    }
}

impl Default for Color {
    fn default() -> Self {
        Color(svgtypes::Color::new_rgb(0, 0, 0))
    }
}

impl From<&Color> for svgtypes::Color {
    fn from(value: &Color) -> Self {
        value.0
    }
}

impl FromStr for Color {
    type Err = NelsieError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Color::new(svgtypes::Color::from_str(s).map_err(|_| {
            NelsieError::Parsing(format!("Invalid color: '{s}'"))
        })?))
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.alpha == 255 {
            write!(
                f,
                "#{:02x}{:02x}{:02x}",
                self.0.red, self.0.green, self.0.blue
            )
        } else {
            write!(
                f,
                "#{:02x}{:02x}{:02x}{:02x}",
                self.0.red, self.0.green, self.0.blue, self.0.alpha
            )
        }
    }
}