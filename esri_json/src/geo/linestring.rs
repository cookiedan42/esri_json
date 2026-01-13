use crate::geometry::{Coord, LineString};

impl<C: Coord> From<LineString<C>> for geo_types::LineString<f64>
where
    geo_types::Coord<f64>: From<C>,
{
    fn from(val: LineString<C>) -> Self {
        geo_types::LineString::<f64>::from_iter(
            val.0.into_iter().map(Into::<geo_types::Coord>::into),
        )
    }
}

impl<C: Coord> From<geo_types::LineString<f64>> for LineString<C>
where
    C: From<geo_types::Coord<f64>>,
{
    fn from(line_string: geo_types::LineString<f64>) -> Self {
        let a: Vec<C> = line_string
            .0
            .into_iter()
            .map(From::<geo_types::Coord<f64>>::from)
            .collect::<Vec<_>>();
        LineString::<C>::new(a)
    }
}
