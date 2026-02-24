use crate::geo_types_n;
use crate::geometry::{Coord, MultiPoint};

impl<C: Coord> From<MultiPoint<C>> for geo_types_n::MultiPoint<C> {
    fn from(value: MultiPoint<C>) -> Self {
        Self(
            value
                .points()
                .iter()
                .map(|&c| geo_types_n::Point(c))
                .collect::<Vec<_>>(),
        )
    }
}

impl<C: Coord> From<geo_types_n::MultiPoint<C>> for MultiPoint<C> {
    fn from(value: geo_types_n::MultiPoint<C>) -> Self {
        Self::new(value.0.iter().map(|f| f.0).collect::<Vec<_>>(), None)
    }
}
