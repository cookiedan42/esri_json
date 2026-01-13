use crate::geometry::{Coord, LineString, SpatialReference};
use serde::{Deserialize, Serialize};

/// Representation of a [Polygon](https://developers.arcgis.com/web-scene-specification/objects/polygon_geometry/)
///
/// Outer rings are oriented clockwise, while holes are oriented counterclockwise.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Polygon<C: Coord> {
    #[serde(rename = "spatialReference")]
    spatial_reference: Option<SpatialReference>,
    rings: Vec<LineString<C>>, // must be closed,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasM")]
    has_m: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasZ")]
    has_z: Option<bool>,
}

impl<C: Coord> Polygon<C> {
    pub fn new(rings: Vec<LineString<C>>, spatial_reference: Option<SpatialReference>) -> Self {
        Self {
            spatial_reference,
            rings,
            has_m: C::has_m_field(),
            has_z: C::has_z_field(),
        }
    }
    pub fn rings(&self) -> &Vec<LineString<C>> {
        &self.rings
    }
    pub fn set_spatial_reference(&mut self, spatial_reference: Option<SpatialReference>) {
        self.spatial_reference = spatial_reference;
    }
    pub fn set_z(self, z: f64) -> Self {
        Self {
            spatial_reference: self.spatial_reference,
            rings: self.rings.into_iter().map(|c| c.set_z(z)).collect(),
            has_m: C::has_m_field(),
            has_z: C::has_z_field(),
        }
    }
}

impl<C: Coord> IntoIterator for Polygon<C> {
    type Item = LineString<C>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rings.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::*;
    use esri_json_test_fixtures::polygon::*;

    #[test]
    fn empty_polygon() {
        let de: Polygon<CoordXy> = serde_json::from_str(&empty()).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Polygon<CoordXy> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[test]
    fn polygon() {
        let de: Polygon<CoordXy> = serde_json::from_str(&xy()).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Polygon<CoordXy> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);

        let de: Polygon<CoordXyz> = serde_json::from_str(&xyz()).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        println!("{}", ser);
        let serde: Polygon<CoordXyz> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
