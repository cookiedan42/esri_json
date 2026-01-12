//! Representation of a [Material](https://developers.arcgis.com/web-scene-specification/objects/material/)
//!
//! The material used to shade the geometry

pub use crate::common::Color;
use serde::{Deserialize, Serialize};

/// The material used to shade the geometry
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Material {
    color: Option<Color>,
    transparency: Option<f64>,
}

/// Builder pattern
impl Material {
    /// Color is represented as a three or four-element array.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    /// A value between 100 (full transparency) and 0 (full opacity). Ignored if no color is specified.
    pub fn transparency(mut self, transparency: f64) -> Result<Self, String> {
        if !(0.0..=100.0).contains(&transparency) {
            Err(format!(
                "Transparency must be a value between 0.0 and 100.0, got {}",
                transparency
            ))
        } else {
            self.transparency = Some(transparency);
            Ok(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let material = r#"{
            "color": [
                255,
                0,
                0
            ],
            "transparency": 20
        }"#;
        let de: Material = serde_json::from_str(material).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Material = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
