use crate::geo_types_n;

use super::Coord;
use geo_traits::RectTrait;

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub struct Rect<C: Coord> {
    min: C,
    max: C,
}
impl<C: Coord> Rect<C> {
    fn new(min: C, max: C) -> Self {
        Self { min, max }
    }
    pub fn to_polygon(&self) -> geo_types_n::Polygon<C> {
        geo_types_n::Polygon::new(
            geo_types_n::LineString(vec![
                self.min,
                C::from_coord_fields(self.max.x(), self.min.y(), self.max.z(), None),
                self.max,
                C::from_coord_fields(self.min.x(), self.max.y(), self.max.z(), None),
                self.min,
            ]),
            vec![],
        )
    }
}

impl<C: Coord> RectTrait for Rect<C> {
    type CoordType<'a>
        = C
    where
        C: 'a;

    fn min(&self) -> C {
        self.min
    }
    fn max(&self) -> C {
        self.max
    }
}
impl<C: Coord> From<&Rect<C>> for geo_types::Rect<f64>
where
    geo_types::Coord<f64>: From<C>,
{
    fn from(val: &Rect<C>) -> Self {
        Self::new(val.min, val.max)
    }
}
impl<C: Coord> From<&geo_types::Rect<f64>> for Rect<C>
where
    C: From<geo_types::Coord<f64>>,
{
    fn from(val: &geo_types::Rect<f64>) -> Self {
        Self::new(val.min().into(), val.max().into())
    }
}
impl_from!(geo_types::Rect<f64>, Rect<C>);
impl_from!(Rect<C>, geo_types::Rect<f64>);
