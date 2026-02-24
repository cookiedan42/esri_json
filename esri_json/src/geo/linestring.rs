//! The LineString primitive of esri_json can be mapped to the LineString type of geo_types_n

use crate::geo_types_n;
use crate::geometry::{Coord, LineString};

impl<C: Coord> From<LineString<C>> for geo_types_n::LineString<C> {
    fn from(value: LineString<C>) -> Self {
        geo_types_n::LineString::<C>(value.0)
    }
}

impl<C: Coord> From<geo_types_n::LineString<C>> for LineString<C> {
    fn from(value: geo_types_n::LineString<C>) -> Self {
        LineString::<C>::new(value.0)
    }
}
impl<C: Coord> From<&LineString<C>> for geo_types_n::LineString<C> {
    fn from(value: &LineString<C>) -> Self {
        geo_types_n::LineString::<C>(value.0.clone())
    }
}

impl<C: Coord> From<&geo_types_n::LineString<C>> for LineString<C> {
    fn from(value: &geo_types_n::LineString<C>) -> Self {
        LineString::<C>::new(value.0.clone())
    }
}
