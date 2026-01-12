//! Representation of a [Simple Fill Symbol](https://developers.arcgis.com/web-map-specification/objects/esriSFS_symbol/)
//!
//! [`EsriSFS`] and re-exported types used to represent a Simple Fill Symbol
//!

use crate::common::marker_type;
use serde::{Deserialize, Serialize};

pub use crate::common::Color;
pub use crate::webmap::esri_sls::EsriSLS as Outline;

marker_type!(T, "esriSFS");

/// Representation of a [Simple Fill Symbol](https://developers.arcgis.com/web-map-specification/objects/esriSFS_symbol/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct EsriSFS {
    #[serde(rename = "type")]
    _type: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outline: Option<Outline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Style>,
}

/// Builder pattern
impl EsriSFS {
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    pub fn outline(mut self, outline: Outline) -> Self {
        self.outline = Some(outline);
        self
    }
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
}

/// Representation of the fill style of a [Simple Fill Symbol](https://developers.arcgis.com/web-map-specification/objects/esriSFS_symbol/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
#[allow(non_camel_case_types)]
pub enum Style {
    esriSFSBackwardDiagonal,
    esriSFSCross,
    esriSFSDiagonalCross,
    esriSFSForwardDiagonal,
    esriSFSHorizontal,
    esriSFSNull,
    esriSFSSolid,
    esriSFSVertical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_fill_symbol_example() {
        let simple_fill_symbol = r#"{
            "type": "esriSFS",
            "style": "esriSFSSolid",
            "color": [
                115,
                76,
                0,
                255
            ],
            "outline": {
                "type": "esriSLS",
                "style": "esriSLSSolid",
                "color": [
                110,
                110,
                110,
                255
                ],
                "width": 1
            }
        }"#;
        let de: EsriSFS = serde_json::from_str(simple_fill_symbol).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: EsriSFS = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}

/*

Simple Fill Symbol (esriSFS)

Simple fill symbols that can be used to symbolize polygon geometries.

Referenced by: GeoRSS Layer (GeoRSS), Predominance Renderer, symbol, Video Layer Drawing Info
Properties
Property 	Details
color 	Color is represented as a four-element array. The four elements represent values for red, green, blue, and alpha in that order. Values range from 0 through 255. If color is undefined for a symbol, the color value is null.
outline 	Sets the outline of the fill symbol.
style 	String value representing the simple fill symbol type.
Valid values: esriSFSBackwardDiagonal, esriSFSCross, esriSFSDiagonalCross, esriSFSForwardDiagonal, esriSFSHorizontal, esriSFSNull, esriSFSSolid, esriSFSVertical
type 	Specifies the type of symbol used.
Valid value of this property esriSFS
Simple Fill Symbol Example


  */
