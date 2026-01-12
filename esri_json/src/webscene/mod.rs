//! Tools for working with the [Web Scene Specification](https://developers.arcgis.com/web-scene-specification/)

pub(crate) mod common;

pub mod layers;
pub use layers::{
    extrude_symbol_3d_layer, fill_symbol_3d_layer, icon_symbol_3d_layer, line_symbol_3d_layer,
    object_symbol_3d_layer, path_symbol_3d_layer, text_symbol_3d_layer, water_symbol_3d_layer,
};

pub mod symbol_3d;
pub use symbol_3d::{
    label_symbol_3d, line_symbol_3d, mesh_symbol_3d, point_symbol_3d, polygon_symbol_3d,
};
