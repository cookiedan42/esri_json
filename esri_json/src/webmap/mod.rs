//! Tools for working with the [Web Map Specification](https://developers.arcgis.com/web-map-specification/)
pub(crate) mod symbol;
pub use symbol::{esri_sfs, esri_sls, esri_sms};
pub(crate) mod common;
pub use crate::common::Color;
