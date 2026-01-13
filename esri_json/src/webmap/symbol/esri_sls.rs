//! Representation of a [Simple Line Symbol](https://developers.arcgis.com/web-map-specification/objects/esriSLS_symbol/)
//!
//! [`EsriSLS`] and re-exported types used to represent a Simple Line Symbol
//!  

use crate::common::marker_type;
use serde::{Deserialize, Serialize};

pub use crate::common::Color;
pub use crate::webmap::common::{Marker, Placement};

marker_type!(T, "esriSLS");

/// Representation of a [Simple Line Symbol](https://developers.arcgis.com/web-map-specification/objects/esriSLS_symbol/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct EsriSLS {
    #[serde(rename = "type")]
    _type: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<Marker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Style>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
}

/// Builder pattern
impl EsriSLS {
    pub const fn builder() -> Self {
        Self {
            _type: T,
            color: None,
            marker: None,
            style: None,
            width: None,
        }
    }

    pub const fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    pub const fn marker(mut self, marker: Marker) -> Self {
        self.marker = Some(marker);
        self
    }
    pub const fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
    pub const fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }
}

/// Representation of the line style of a [Simple Line Symbol](https://developers.arcgis.com/web-map-specification/objects/esriSLS_symbol/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
#[allow(non_camel_case_types)]
pub enum Style {
    esriSLSDash,
    esriSLSDashDot,
    esriSLSDashDotDot,
    esriSLSDot,
    esriSLSLongDash,
    esriSLSLongDashDot,
    esriSLSNull,
    esriSLSShortDash,
    esriSLSShortDashDot,
    esriSLSShortDashDotDot,
    esriSLSShortDot,
    esriSLSSolid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_line_symbol_example() {
        let simple_line_symbol = r#"{
            "type": "esriSLS",
            "style": "esriSLSDot",
            "color": [
                115,
                76,
                0,
                255
            ],
            "width": 1
        }"#;
        let de: EsriSLS = serde_json::from_str(simple_line_symbol).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: EsriSLS = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[test]
    fn simple_line_symbol_example2() {
        let simple_line_symbol = r#"{
            "type": "esriSLS",
            "color": [
            226,
            119,
            40,
            255
            ],
            "width": 4,
            "style": "esriSLSSolid",
            "marker": {
            "placement": "begin",
            "type": "line-marker",
            "style": "arrow",
            "color": null
            }
        }"#;
        let de: EsriSLS = serde_json::from_str(simple_line_symbol).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: EsriSLS = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
