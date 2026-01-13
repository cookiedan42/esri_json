//! Primitive type for a Polyline

use super::coord::Coord;

use serde::{Deserialize, Serialize};

/// An ordered Collection of [`Coord`], used as part of [`Polyline`](crate::geometry::Polyline) and [`Polygon`](crate::geometry::Polygon)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LineString<C: Coord>(pub(crate) Vec<C>);

impl<C: Coord> LineString<C> {
    pub fn new(points: Vec<C>) -> Self {
        Self(points)
    }
    pub fn points(&self) -> &Vec<C> {
        &self.0
    }
    pub fn set_z(self, z: f64) -> Self {
        Self(self.0.iter().map(|c| c.set_z(z)).collect())
    }
}
impl<C: Coord> IntoIterator for LineString<C> {
    type Item = C;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
