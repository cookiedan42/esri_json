use crate::geometry::{Coord, LineString};

impl<N: Coord> From<&LineString<N>> for geo_types::LineString<f64>
where
    for<'a> &'a N: Into<geo_types::Coord<f64>>,
{
    fn from(val: &LineString<N>) -> Self {
        geo_types::LineString::<f64>::from_iter(
            val.points().iter().map(Into::<geo_types::Coord>::into),
        )
    }
}

impl<N: Coord> From<LineString<N>> for geo_types::LineString<f64>
where
    for<'a> &'a N: Into<geo_types::Coord<f64>>,
{
    fn from(val: LineString<N>) -> Self {
        (&val).into()
    }
}

impl<N: Coord> From<&geo_types::LineString<f64>> for LineString<N>
where
    for<'a> N: From<&'a geo_types::Coord<f64>>,
{
    fn from(line_string: &geo_types::LineString<f64>) -> Self {
        let a: Vec<N> = line_string
            .coords()
            .map(From::<&geo_types::Coord<f64>>::from)
            .collect::<Vec<_>>();
        LineString::<N>::new(a)
    }
}

impl<N: Coord> From<geo_types::LineString<f64>> for LineString<N>
where
    for<'a> N: From<&'a geo_types::Coord<f64>>,
{
    fn from(line_string: geo_types::LineString<f64>) -> Self {
        (&line_string).into()
    }
}
