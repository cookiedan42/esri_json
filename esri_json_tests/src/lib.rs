//! Tests for the esri_json crate
//! Documenting how to use the crate and that all the types are reachable by the user

#[cfg(test)]
mod geo;
#[cfg(test)]
mod geometry;
#[cfg(test)]
mod graphics_layer;
#[cfg(test)]
mod webmap;
#[cfg(test)]
mod webscene;

#[allow(dead_code)]
pub(crate) mod fixtures;
