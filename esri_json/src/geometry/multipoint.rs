use crate::geometry::{Coord, SpatialReference};
use serde::{Deserialize, Serialize};

/// Representation of a [MultiPoint](https://developers.arcgis.com/web-scene-specification/objects/multipoint_geometry/)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MultiPoint<N: Coord> {
    #[serde(rename = "spatialReference")]
    spatial_reference: Option<SpatialReference>,
    points: Vec<N>,
    // ids: Option<Vec<u32>>,
}

impl<N: Coord> MultiPoint<N> {
    pub fn new(points: Vec<N>, spatial_reference: Option<SpatialReference>) -> Self {
        Self {
            spatial_reference,
            points,
        }
    }
    pub fn points(&self) -> &Vec<N> {
        &self.points
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
