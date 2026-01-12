//! Representation of a [LinePattern](https://developers.arcgis.com/web-scene-specification/objects/linePattern/)
//!
//! A pattern used to render a line.

use crate::common::marker_type;
use serde::{Deserialize, Serialize};

marker_type!(T, "style");

/// A pattern used to render a line
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct LinePatternInternal {
    style: LinePattern,
    #[serde(rename = "type")]
    _type: T,
}

impl From<LinePattern> for LinePatternInternal {
    fn from(style: LinePattern) -> Self {
        Self { _type: T, style }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum LinePattern {
    Dash,
    DashDot,
    Dot,
    LongDash,
    LongDashDot,
    LongDashDotDot,
    None,
    ShortDash,
    ShortDashDot,
    ShortDashDotDot,
    ShortDot,
    Solid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_style_pattern_3d() {
        let line_style_pattern_3d = r#"
        {
            "type": "style",
            "style": "dash-dot"
        }
        "#;
        let de: LinePatternInternal = serde_json::from_str(line_style_pattern_3d).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: LinePatternInternal = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
