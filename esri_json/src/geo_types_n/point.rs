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
    use crate::geo_types_n::CoordNumber;
    use crate::geometry::*;
    use rstest::rstest;

    #[rstest]
    #[case::f32(std::marker::PhantomData::<f32>)]
    #[case::f64(std::marker::PhantomData::<f64>)]
    fn points<T>(#[case] _phantom: std::marker::PhantomData<T>)
    where
        T: CoordNumber + From<f32>,
    {
        let _p: geo_types::Point<T> =
            Point::<CoordXy<T>>(CoordXy::from_coord_fields(0.0, 0.0, None, None)).into();
        let _p: geo_types::Point<T> =
            Point::<CoordXyz<T>>(CoordXyz::from_coord_fields(0.0, 0.0, Some(0.0), None)).into();
        let _p: geo_types::Point<T> =
            Point::<CoordXym<T>>(CoordXym::from_coord_fields(0.0, 0.0, None, Some(0.0))).into();
        let _p: geo_types::Point<T> =
            Point::<CoordXyzm<T>>(CoordXyzm::from_coord_fields(0.0, 0.0, Some(0.0), Some(0.0)))
                .into();
    }
}
