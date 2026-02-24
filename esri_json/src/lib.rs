//! # Core esri_json types and conversion to geo_types  
//! Includes a smattering of webscene specification types
//! and the basic geometries  
//!
//! ```
//! use esri_json::geometry::{Coord,CoordXy,Point};
//! use esri_json::js_sdk::graphic;
//! use esri_json::webmap::esri_sms::*;
//! use serde_json::Map;
//!
//! let geometry: Point<CoordXy> = Point::new(CoordXy::new(0.0, 0.0), None);
//!
//! let sms = EsriSMS::default()
//!     .angle(1.0)
//!     .color(Color::Rgb(1, 1, 1))
//!     .outline(Outline::default())
//!     .size(1.0)
//!     .style(Style::esriSMSCircle)
//!     .xoffset(1.0)
//!     .yoffset(1.0);
//!
//! let _g = graphic::Graphic::<Point<CoordXy>>::new(geometry)
//!     .attributes(Map::new())
//!     .symbol(sms);
//! ```
//!

pub(crate) mod common;

#[cfg(feature = "geo_types")]
pub mod geo;

pub mod geojson;

pub mod geometry;
pub mod js_sdk;
pub mod webmap;
pub mod webscene;

pub mod geo_types_n;
