use geo_traits::PointTrait;

use super::Coord;

#[derive(Eq, PartialEq, Clone, Copy, Hash, Default)]
pub struct Point<C: Coord>(pub C);

// TODO: this is not working becuase of self conversion
// implement individual conversions for each type as we encounter them

// impl<IC, C> From<IC> for Point<C>
// where
//     for<'a> &'a IC: Into<C>,
//     C: Coord,
// {
//     fn from(val: IC) -> Self {
//         Point((&val).into())
//     }
// }

impl<C: Coord> PointTrait for Point<C> {
    type CoordType<'a>
        = C
    where
        C: 'a;

    fn coord(&self) -> Option<Self::CoordType<'_>> {
        Some(self.0)
    }
}

impl<C: Coord> From<&Point<C>> for geo_types::Point<C::T>
where
    geo_types::Coord<C::T>: From<C>,
{
    fn from(val: &Point<C>) -> Self {
        Self(val.0.into())
    }
}
impl<C: Coord> From<&geo_types::Point<C::T>> for Point<C>
where
    C: From<geo_types::Coord<C::T>>,
{
    fn from(val: &geo_types::Point<C::T>) -> Self {
        Self(val.0.into())
    }
}
impl_from!(geo_types::Point<C::T>, Point<C>);
impl_from!(Point<C>, geo_types::Point<C::T>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CoordNumber;
    use crate::geometry::*;
    use esri_json_macro::test_all_coord_types;

    #[test_all_coord_types]
    fn points<C>() {
        let _p: geo_types::Point<C::T> =
            Point::<C>(C::from_coord_fields(0.0, 0.0, None, None)).into();
        let _p: geo_types::Point<C::T> =
            Point::<C>(C::from_coord_fields(0.0, 0.0, Some(0.0), None)).into();
        let _p: geo_types::Point<C::T> =
            Point::<C>(C::from_coord_fields(0.0, 0.0, None, Some(0.0))).into();
        let _p: geo_types::Point<C::T> =
            Point::<C>(C::from_coord_fields(0.0, 0.0, Some(0.0), Some(0.0))).into();
    }
}
