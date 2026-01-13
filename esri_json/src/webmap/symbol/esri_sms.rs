//! Representation of a [Simple Marker Symbol](https://developers.arcgis.com/web-map-specification/objects/esriSMS_symbol/)
//!
//! [`EsriSMS`] and re-exported types used to represent a Simple Marker Symbol
//!
use crate::common::marker_type;
use serde::{Deserialize, Serialize};

pub use crate::common::Color;
pub use crate::webmap::esri_sls::EsriSLS as Outline;

marker_type!(T, "esriSMS");

/// Representation of a [Simple Marker Symbol](https://developers.arcgis.com/web-map-specification/objects/esriSMS_symbol/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct EsriSMS {
    #[serde(rename = "type")]
    _type: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    angle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outline: Option<Outline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Style>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xoffset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    yoffset: Option<f64>,
}

/// Builder pattern
impl EsriSMS {
    pub const fn builder() -> Self {
        Self {
            _type: T,
            angle: None,
            color: None,
            outline: None,
            size: None,
            style: None,
            xoffset: None,
            yoffset: None,
        }
    }
    pub const fn angle(mut self, angle: f64) -> Self {
        self.angle = Some(angle);
        self
    }
    pub const fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    pub const fn outline(mut self, outline: Outline) -> Self {
        self.outline = Some(outline);
        self
    }
    pub const fn size(mut self, size: f64) -> Self {
        self.size = Some(size);
        self
    }
    pub const fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
    pub const fn xoffset(mut self, xoffset: f64) -> Self {
        self.xoffset = Some(xoffset);
        self
    }
    pub const fn yoffset(mut self, yoffset: f64) -> Self {
        self.yoffset = Some(yoffset);
        self
    }
}

/// Representation of the marker style of a [Simple Marker Symbol](https://developers.arcgis.com/web-map-specification/objects/esriSMS_symbol/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
#[allow(non_camel_case_types)]
pub enum Style {
    esriSMSCircle,
    esriSMSCross,
    esriSMSDiamond,
    esriSMSSquare,
    esriSMSTriangle,
    esriSMSX,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_marker_symbol_example() {
        // this differs from the example in the spec
        // the example has a type field on the outline, but the spec does not
        let simple_marker_symbol = r#"{
            "type": "esriSMS",
            "style": "esriSMSSquare",
            "color": [
                76,
                115,
                0,
                255
            ],
            "size": 8,
            "angle": 0,
            "xoffset": 0,
            "yoffset": 0,
            "outline": {
                "type": "esriSLS",
                "color": [
                152,
                230,
                0,
                255
                ],
                "width": 1
            }
        }"#;
        let de: EsriSMS = serde_json::from_str(simple_marker_symbol).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: EsriSMS = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
