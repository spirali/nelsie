use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Size {
    Points(f32),
    Percent(f32),
    Auto,
}

#[derive(Debug)]
pub(crate) struct Color(svgtypes::Color);

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let color = svgtypes::Color::from_str(&value)
            .map_err(|_| serde::de::Error::custom("Invalid color"))?;
        Ok(Color(color))
    }
}

impl From<&Color> for svgtypes::Color {
    fn from(value: &Color) -> Self {
        value.0
    }
}
