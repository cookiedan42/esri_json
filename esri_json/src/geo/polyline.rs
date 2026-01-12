use crate::geometry::{Coord, LineString, Polyline};

// The only valid way to convert an esri Polyline to geo_types is to make it a MultiLineString
// Subsequent decomposition to LineString can be done by user
// But we can safely convert any of the 3 line type geometries to an esri Polyline

impl<N: Coord> From<&Polyline<N>> for geo_types::MultiLineString<f64>
where
    for<'a> &'a N: Into<geo_types::Coord<f64>>,
{
    fn from(val: &Polyline<N>) -> Self {
        let a = val.paths().iter().map(|path| {
            path.points()
                .iter()
                .map(Into::<geo_types::Coord>::into)
                .collect::<Vec<_>>()
        });
        geo_types::MultiLineString::<f64>::from_iter(a)
    }
}

impl<N: Coord> From<Polyline<N>> for geo_types::MultiLineString<f64>
where
    for<'a> &'a N: Into<geo_types::Coord<f64>>,
{
    fn from(val: Polyline<N>) -> Self {
        (&val).into()
    }
}

impl<N: Coord> From<&geo_types::MultiLineString<f64>> for Polyline<N>
where
    for<'a> N: From<&'a geo_types::Coord<f64>>,
{
    fn from(line_string: &geo_types::MultiLineString<f64>) -> Self {
        let a = line_string
            .iter()
            .map(Into::<LineString<N>>::into)
            .collect::<Vec<_>>();
        Polyline::<N>::new(a, None)
    }
}

impl<N: Coord> From<geo_types::MultiLineString<f64>> for Polyline<N>
where
    for<'a> N: From<&'a geo_types::Coord<f64>>,
{
    fn from(line_string: geo_types::MultiLineString<f64>) -> Self {
        (&line_string).into()
    }
}

impl<N: Coord> From<&geo_types::LineString<f64>> for Polyline<N>
where
    for<'a> N: From<&'a geo_types::Coord<f64>>,
{
    fn from(line_string: &geo_types::LineString<f64>) -> Self {
        let a = vec![line_string.into()];
        Polyline::<N>::new(a, None)
    }
}

impl<N: Coord> From<geo_types::LineString<f64>> for Polyline<N>
where
    for<'a> N: From<&'a geo_types::Coord<f64>>,
{
    fn from(line_string: geo_types::LineString<f64>) -> Self {
        (&line_string).into()
    }
}

impl<N: Coord> From<&geo_types::Line<f64>> for Polyline<N>
where
    for<'a> N: From<&'a geo_types::Coord<f64>>,
{
    fn from(line_string: &geo_types::Line<f64>) -> Self {
        let a = vec![LineString::<N>::new(vec![
            (&line_string.start).into(),
            (&line_string.end).into(),
        ])];
        Polyline::<N>::new(a, None)
    }
}

impl<N: Coord> From<geo_types::Line<f64>> for Polyline<N>
where
    for<'a> N: From<&'a geo_types::Coord<f64>>,
{
    fn from(line_string: geo_types::Line<f64>) -> Self {
        (&line_string).into()
    }
}
