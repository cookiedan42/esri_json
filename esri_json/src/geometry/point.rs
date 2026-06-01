use crate::geometry::{Coord, SpatialReference};
use geo_traits::CoordTrait;
use serde::{Deserialize, Serialize};

/// Representation of a [Point](https://developers.arcgis.com/web-scene-specification/objects/point_geometry/)  
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(
    into = "PointHelper<C>",
    from = "PointHelper<C>",
    bound(
        serialize = "<C as CoordTrait>::T: Serialize",
        deserialize = "<C as CoordTrait>::T: Deserialize<'de>"
    )
)]
pub struct Point<C: Coord> {
    #[serde(rename = "spatialReference")]
    spatial_reference: Option<SpatialReference>,
    coord: C,
    has_m: Option<bool>,
    has_z: Option<bool>,
}

impl<C: Coord> Point<C> {
    pub fn new(coord: C, spatial_reference: Option<SpatialReference>) -> Self {
        Self {
            coord,
            spatial_reference,
            has_m: C::has_m_field(),
            has_z: C::has_z_field(),
        }
    }
    pub fn coord(&self) -> &C {
        &self.coord
    }
    pub fn x(&self) -> <C as CoordTrait>::T {
        self.coord.x()
    }
    pub fn y(&self) -> <C as CoordTrait>::T {
        self.coord.y()
    }
    pub fn z(&self) -> Option<<C as CoordTrait>::T> {
        self.coord.z()
    }
    pub fn m(&self) -> Option<<C as CoordTrait>::T> {
        self.coord.m()
    }
    pub fn set_spatial_reference(&mut self, spatial_reference: Option<SpatialReference>) {
        self.spatial_reference = spatial_reference;
    }
    pub fn set_z(self, z: <C as CoordTrait>::T) -> Self {
        Self {
            coord: self.coord.set_z(z),
            spatial_reference: self.spatial_reference,
            has_m: C::has_m_field(),
            has_z: C::has_z_field(),
        }
    }
}

/// Helper type for serialization, because Point holds each coord value separately
#[derive(Serialize, Deserialize)]
#[serde(bound(
    serialize = "<C as CoordTrait>::T: Serialize",
    deserialize = "<C as CoordTrait>::T: Deserialize<'de>"
))]
struct PointHelper<C: Coord> {
    #[serde(rename = "spatialReference")]
    spatial_reference: Option<SpatialReference>,
    x: <C as CoordTrait>::T,
    y: <C as CoordTrait>::T,
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<<C as CoordTrait>::T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    m: Option<<C as CoordTrait>::T>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasM")]
    has_m: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasZ")]
    has_z: Option<bool>,
}

impl<C: Coord> From<PointHelper<C>> for Point<C> {
    fn from(helper: PointHelper<C>) -> Self {
        Point {
            spatial_reference: helper.spatial_reference,
            coord: C::from_coord_fields(helper.x, helper.y, helper.z, helper.m),
            has_m: C::has_m_field(),
            has_z: C::has_z_field(),
        }
    }
}

impl<C: Coord> From<Point<C>> for PointHelper<C> {
    fn from(val: Point<C>) -> Self {
        PointHelper {
            spatial_reference: val.spatial_reference,
            x: val.coord.x(),
            y: val.coord.y(),
            z: val.coord.z(),
            m: val.coord.m(),
            has_m: C::has_m_field(),
            has_z: C::has_z_field(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CoordNumber;
    use crate::geometry::*;
    use esri_json_macro::test_all_coord_types;
    use esri_json_test_fixtures::point::*;

    #[test_all_coord_types]
    fn points<C>(#[case] _phantom: std::marker::PhantomData<C>) {
        let xy = point_xy();
        let xyz = point_xyz();
        let xym = point_xym();
        let xyzm = point_xyzm();

        let de: Point<C> = serde_json::from_str(&xy).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Point<C> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);

        let de: Point<C> = serde_json::from_str(&xyz).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Point<C> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);

        let de: Point<C> = serde_json::from_str(&xym).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Point<C> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);

        let de: Point<C> = serde_json::from_str(&xyzm).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Point<C> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
