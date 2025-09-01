use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color(svgtypes::Color);

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let rgba = [self.0.red, self.0.green, self.0.blue, self.0.alpha];
        state.write_u32(u32::from_ne_bytes(rgba));
    }
}

impl Color {
    pub fn new(color: svgtypes::Color) -> Self {
        Color(color)
    }

    pub fn as_f32s(&self) -> [f32; 3] {
        [
            self.0.red as f32 / 255.0,
            self.0.green as f32 / 255.0,
            self.0.blue as f32 / 255.0,
        ]
    }

    pub fn alpha(&self) -> u8 {
        self.0.alpha
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
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Color::new(svgtypes::Color::from_str(s).map_err(|_| {
            crate::Error::parsing_err(format!("Invalid color: '{s}'"))
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
