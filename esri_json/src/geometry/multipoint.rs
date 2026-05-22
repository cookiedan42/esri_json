use crate::geometry::{Coord, SpatialReference};
use geo_traits::CoordTrait;
use serde::{Deserialize, Serialize};

/// Representation of a [MultiPoint](https://developers.arcgis.com/web-scene-specification/objects/multipoint_geometry/)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(bound(serialize = "C: Serialize", deserialize = "C: Deserialize<'de>"))]
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
    pub fn set_z(self, z: <C as CoordTrait>::T) -> Self {
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
    use crate::geo_types_n::CoordNumber;
    use crate::geometry::*;
    use esri_json_test_fixtures::multipoint::*;
    use rstest::rstest;
    use serde::{Serialize, de::DeserializeOwned};

    #[rstest]
    #[case::f32(std::marker::PhantomData::<f32>)]
    #[case::f64(std::marker::PhantomData::<f64>)]
    fn multipoint_xy<T>(#[case] _phantom: std::marker::PhantomData<T>)
    where
        T: CoordNumber + From<f32> + Serialize + DeserializeOwned,
    {
        let xy = xy();

        let de: MultiPoint<CoordXy<T>> = serde_json::from_str(&xy).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: MultiPoint<CoordXy<T>> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[rstest]
    #[case::f32(std::marker::PhantomData::<f32>)]
    #[case::f64(std::marker::PhantomData::<f64>)]
    fn multipoint_xyz<T>(#[case] _phantom: std::marker::PhantomData<T>)
    where
        T: CoordNumber + From<f32> + Serialize + DeserializeOwned,
    {
        let xyz = xyz();

        let de: MultiPoint<CoordXyz<T>> = serde_json::from_str(&xyz).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: MultiPoint<CoordXyz<T>> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
