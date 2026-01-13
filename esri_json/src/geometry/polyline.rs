use crate::geometry::{Coord, LineString, SpatialReference};
use serde::{Deserialize, Serialize, ser::SerializeMap};
/// Representation of a [Polyline](https://developers.arcgis.com/web-scene-specification/objects/polyline_geometry/)
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Polyline<C: Coord> {
    #[serde(rename = "spatialReference")]
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
    pub fn set_z(self, z: f64) -> Self {
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

impl<C: Coord + Serialize> Serialize for Polyline<C> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;

        if C::has_m() {
            map.serialize_entry("hasM", &true)?;
        }
        if C::has_z() {
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
