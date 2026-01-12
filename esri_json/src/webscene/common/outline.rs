//! Representation of a [Outline](https://developers.arcgis.com/web-scene-specification/objects/outline/)
//!
//! The outline of the symbol layer.

use crate::webscene::common::LinePatternInternal;
use serde::{Deserialize, Serialize};

pub use crate::common::Color;
pub use crate::webscene::common::LineCap;
pub use crate::webscene::common::LinePattern;

/// Representation of a [Outline](https://developers.arcgis.com/web-scene-specification/objects/outline/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Outline {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<LinePatternInternal>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "patternCap")]
    pattern_cap: Option<LineCap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transparency: Option<f64>,
}

/// Builder pattern
impl Outline {
    /// Color is represented as a three or four-element array.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    /// A pattern used to render a line.
    pub fn pattern(mut self, pattern: LinePattern) -> Self {
        self.pattern = Some(pattern.into());
        self
    }
    /// Shape of the tips at the start and end of each line geometry. This also applies to the tips of each pattern segment along the line.
    pub fn pattern_cap(mut self, pattern_cap: LineCap) -> Self {
        self.pattern_cap = Some(pattern_cap);
        self
    }
    /// Outline size in points, positive only
    pub fn size(mut self, size: f64) -> Result<Self, String> {
        if size < 0.0 {
            Err(format!("Size must be positive, got {}", size))
        } else {
            self.size = Some(size);
            Ok(self)
        }
    }
    /// The value has to lie between 100 (full transparency) and 0 (full opacity).
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
