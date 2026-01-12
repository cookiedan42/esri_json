use crate::geometry::{Coord, SpatialReference};
use serde::{Deserialize, Serialize};

/// Representation of a [Point](https://developers.arcgis.com/web-scene-specification/objects/point_geometry/)  
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Point<N: Coord> {
    #[serde(rename = "spatialReference")]
    spatial_reference: Option<SpatialReference>,
    #[serde(flatten)]
    coord: N,
}

impl<N: Coord> Point<N> {
    pub fn new(coord: N, spatial_reference: Option<SpatialReference>) -> Self {
        Self {
            coord,
            spatial_reference,
        }
    }
    pub fn x(&self) -> f64 {
        self.coord.x()
    }
    pub fn y(&self) -> f64 {
        self.coord.y()
    }
    pub fn z(&self) -> Option<f64> {
        self.coord.z()
    }
    pub fn m(&self) -> Option<f64> {
        self.coord.m()
    }
}

// #[derive(Serialize, Deserialize)]
// struct PointHelper {
//     #[serde(rename = "spatialReference")]
//     spatial_reference: Option<SpatialReference>,
//     x: f64,
//     y: f64,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     z: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     m: Option<f64>,
// }

// impl<N: Coord> From<PointHelper> for Point<N> {
//     fn from(helper: PointHelper) -> Self {
//         Point {
//             spatial_reference: helper.spatial_reference,
//             coord: N::from_coord(helper),
//         }
//     }
// }

// impl<N: Coord> From<Point<N>> for PointHelper {
//     fn from(val: Point<N>) -> Self {
//         PointHelper {
//             spatial_reference: val.spatial_reference,
//             x: val.coord.x(),
//             y: val.coord.y(),
//             z: val.coord.z(),
//             m: val.coord.m(),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::*;

    #[test]
    fn points() {
        use esri_json_test_fixtures::point::*;

        let xy = point_xy();
        let xyz = point_xyz();
        let xym = point_xym();
        let xyzm = point_xyzm();

        let de: Point<CoordXy> = serde_json::from_str(&xy).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Point<CoordXy> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);

        let de: Point<CoordXy> = serde_json::from_str(&xyz).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Point<CoordXy> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);

        let de: Point<CoordXy> = serde_json::from_str(&xym).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Point<CoordXy> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);

        let de: Point<CoordXy> = serde_json::from_str(&xyzm).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Point<CoordXy> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
