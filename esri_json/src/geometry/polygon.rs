use crate::geometry::{Coord, LineString, SpatialReference};
use serde::{Deserialize, Serialize};

/// Representation of a [Polygon](https://developers.arcgis.com/web-scene-specification/objects/polygon_geometry/)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Polygon<N: Coord> {
    #[serde(rename = "spatialReference")]
    spatial_reference: Option<SpatialReference>,
    rings: Vec<LineString<N>>, // must be closed
}

impl<N: Coord> Polygon<N> {
    pub fn new(rings: Vec<LineString<N>>, spatial_reference: Option<SpatialReference>) -> Self {
        Self {
            spatial_reference,
            rings,
        }
    }
    pub fn rings(&self) -> &Vec<LineString<N>> {
        &self.rings
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
    fn drop_dim() {
        // cast a xyz to xy
        let de: Polygon<CoordXy> = serde_json::from_str(&xyz()).unwrap();
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
        let serde: Polygon<CoordXyz> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
