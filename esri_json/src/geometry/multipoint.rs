use crate::geometry::{Coord, SpatialReference};
use serde::{Deserialize, Serialize};

/// Representation of a [MultiPoint](https://developers.arcgis.com/web-scene-specification/objects/multipoint_geometry/)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MultiPoint<C: Coord> {
    #[serde(rename = "spatialReference")]
    spatial_reference: Option<SpatialReference>,
    points: Vec<C>,
    // ids: Option<Vec<u32>
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasM")]
    has_m: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasZ")]
    has_z: Option<bool>,
}

impl<C: Coord> MultiPoint<C> {
    pub fn new(points: Vec<C>, spatial_reference: Option<SpatialReference>) -> Self {
        Self {
            spatial_reference,
            points,
            has_m: C::has_m_field(),
            has_z: C::has_z_field(),
        }
    }
    pub fn points(&self) -> &Vec<C> {
        &self.points
    }
    pub fn set_spatial_reference(&mut self, spatial_reference: Option<SpatialReference>) {
        self.spatial_reference = spatial_reference;
    }
    pub fn set_z(self, z: f64) -> Self {
        Self {
            spatial_reference: self.spatial_reference,
            points: self.points.into_iter().map(|c| c.set_z(z)).collect(),
            has_m: C::has_m_field(),
            has_z: C::has_z_field(),
        }
    }
}
impl<C: Coord> IntoIterator for MultiPoint<C> {
    type Item = C;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::*;
    use esri_json_test_fixtures::multipoint::*;

    #[test]
    fn multipoint_xy() {
        let xy = xy();

        let de: MultiPoint<CoordXy> = serde_json::from_str(&xy).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: MultiPoint<CoordXy> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[test]
    fn multipoint_xyz() {
        let xyz = xyz();

        let de: MultiPoint<CoordXyz> = serde_json::from_str(&xyz).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: MultiPoint<CoordXyz> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
