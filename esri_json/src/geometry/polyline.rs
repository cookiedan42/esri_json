use crate::geometry::{Coord, LineString, SpatialReference};
use geo_traits::CoordTrait;
use serde::{Deserialize, Serialize};

/// Representation of a [Polyline](https://developers.arcgis.com/web-scene-specification/objects/polyline_geometry/)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
// #[serde(bound(serialize = "C: Serialize", deserialize = "C: Deserialize<'de>"))]
pub struct Polyline<C: Coord> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "spatialReference")]
    spatial_reference: Option<SpatialReference>,
    paths: Vec<LineString<C>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasM")]
    has_m: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasZ")]
    has_z: Option<bool>,
}

impl<C: Coord> Polyline<C> {
    pub fn new(paths: Vec<LineString<C>>, spatial_reference: Option<SpatialReference>) -> Self {
        Self {
            spatial_reference,
            paths,
            has_m: C::has_m_field(),
            has_z: C::has_z_field(),
        }
    }
    pub fn paths(&self) -> &Vec<LineString<C>> {
        &self.paths
    }
    pub fn set_spatial_reference(&mut self, spatial_reference: Option<SpatialReference>) {
        self.spatial_reference = spatial_reference;
    }
    pub fn set_z(self, z: <C as CoordTrait>::T) -> Self {
        Self {
            spatial_reference: self.spatial_reference,
            paths: self.paths.into_iter().map(|c| c.set_z(z)).collect(),
            has_m: C::has_m_field(),
            has_z: C::has_z_field(),
        }
    }
}

impl<C: Coord> IntoIterator for Polyline<C> {
    type Item = LineString<C>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.paths.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CoordNumber;
    use crate::geometry::*;
    use esri_json_macro::{assert_json_roundtrip, test_all_number_types};
    use esri_json_test_fixtures::polyline::*;

    #[test_all_number_types]
    fn empty_polyline<T>() {
        assert_json_roundtrip!(Polyline<CoordXy<T>>, &empty());
        // z is true and m is true show up regardless
        // assert_json_roundtrip!(Polyline<CoordXyz<T>>, &empty());
        // assert_json_roundtrip!(Polyline<CoordXym<T>>, &empty());
        // assert_json_roundtrip!(Polyline<CoordXyzm<T>>, &empty());
    }

    #[test_all_number_types]
    fn linestring<T>() {
        assert_json_roundtrip!(Polyline<CoordXy<T>>, &line_xy());
        assert_json_roundtrip!(Polyline<CoordXyz<T>>, &line_xyz());
        assert_json_roundtrip!(Polyline<CoordXym<T>>, &line_xym());
        assert_json_roundtrip!(Polyline<CoordXyzm<T>>, &line_xyzm());
        assert_json_roundtrip!(Polyline<CoordXym<T>>, &line_xym_mixed());
    }
}
