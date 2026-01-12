//! Representation of a [LineMarker](https://developers.arcgis.com/web-scene-specification/objects/lineMarker/)
//!
//! Represents markers placed at the start and end of each line geometry, or both. Markers size is proportional to the width of the line.

use crate::common::Color;
use crate::common::marker_type;
use serde::{Deserialize, Serialize};

marker_type!(T, "style");

/// Represents markers placed at the start and end of each line geometry, or both
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct LineMarker {
    color: Option<Color>,
    placement: Option<Placement>,
    style: Option<LineSymbolMarkerStyle>,
    #[serde(rename = "type")]
    _type: T,
}

/// Builder pattern
impl LineMarker {
    /// An option to color the markers differently from the line. By default the markers inherit the line's color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    /// Indicates where the marker is placed
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = Some(placement);
        self
    }
    /// Style of the marker
    pub fn style(mut self, style: LineSymbolMarkerStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// Indicates where the marker is placed.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum Placement {
    Begin,
    BeginEnd,
    End,
}
/// Style of the marker
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum LineSymbolMarkerStyle {
    Arrow,
    Circle,
    Cross,
    Diamond,
    Square,
    X,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let line_marker = r#"{
            "style": "arrow",
            "placement": "begin",
            "type": "style"
        }"#;
        let de: LineMarker = serde_json::from_str(line_marker).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: LineMarker = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
