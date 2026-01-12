//! Primitive type for a Polyline

use super::coord::Coord;

use serde::{Deserialize, Serialize};

/// An ordered Collection of [`Coord`], used as part of [`Polyline`](crate::geometry::Polyline) and [`Polygon`](crate::geometry::Polygon)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LineString<N: Coord>(pub(crate) Vec<N>);

impl<N: Coord> LineString<N> {
    pub fn new(points: Vec<N>) -> Self {
        Self(points)
    }
    pub fn points(&self) -> &Vec<N> {
        &self.0
    }
}
