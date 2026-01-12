use crate::geometry::{Coord, LineString, SpatialReference};
use serde::{Deserialize, Serialize, ser::SerializeMap};
/// Representation of a [Polyline](https://developers.arcgis.com/web-scene-specification/objects/polyline_geometry/)
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Polyline<N: Coord> {
    #[serde(rename = "spatialReference")]
    spatial_reference: Option<SpatialReference>,
    paths: Vec<LineString<N>>,
}

impl<N: Coord> Polyline<N> {
    pub fn new(paths: Vec<LineString<N>>, spatial_reference: Option<SpatialReference>) -> Self {
        Self {
            spatial_reference,
            paths,
        }
    }
    pub fn paths(&self) -> &Vec<LineString<N>> {
        &self.paths
    }
}

impl<N: Coord + Serialize> Serialize for Polyline<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;

        if N::has_m() {
            map.serialize_entry("hasM", &true)?;
        }
        if N::has_z() {
            map.serialize_entry("hasZ", &true)?;
        }
        map.serialize_entry("spatialReference", &self.spatial_reference)?;
        map.serialize_entry("paths", &self.paths)?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::*;
    use esri_json_test_fixtures::polyline::*;

    #[test]
    fn empty_polyline() {
        let de: Polyline<CoordXy> = serde_json::from_str(&empty()).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Polyline<CoordXy> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[test]
    fn linestring() {
        let xy = line_xy();
        let xyz = line_xyz();
        let xym = line_xym();
        let xyzm = line_xyzm();
        let mixed = line_xym_mixed();

        let de: Polyline<CoordXy> = serde_json::from_str(&xy).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Polyline<CoordXy> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);

        let de: Polyline<CoordXyz> = serde_json::from_str(&xyz).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Polyline<CoordXyz> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);

        let de: Polyline<CoordXym> = serde_json::from_str(&xym).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Polyline<CoordXym> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);

        let de: Polyline<CoordXyzm> = serde_json::from_str(&xyzm).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Polyline<CoordXyzm> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);

        let de: Polyline<CoordXym> = serde_json::from_str(&mixed).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        println!("{}", ser);
        let serde: Polyline<CoordXym> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
