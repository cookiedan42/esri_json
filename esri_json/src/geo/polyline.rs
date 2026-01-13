use crate::geometry::{Coord, LineString, Polyline};

// The only valid way to convert an esri Polyline to geo_types is to make it a MultiLineString
// Subsequent decomposition to LineString can be done by user
// But we can safely convert any of the 3 line type geometries to an esri Polyline

impl<C: Coord> From<Polyline<C>> for geo_types::MultiLineString<f64>
where
    geo_types::Coord<f64>: From<C> + From<C>,
{
    fn from(val: Polyline<C>) -> Self {
        let a = val.into_iter().map(|path| {
            path.into_iter()
                .map(Into::<geo_types::Coord>::into)
                .collect::<Vec<_>>()
        });
        geo_types::MultiLineString::<f64>::from_iter(a)
    }
}

impl<C: Coord> From<geo_types::MultiLineString<f64>> for Polyline<C>
where
    C: From<geo_types::Coord<f64>>,
{
    fn from(line_string: geo_types::MultiLineString<f64>) -> Self {
        let a = line_string
            .into_iter()
            .map(Into::<LineString<C>>::into)
            .collect::<Vec<_>>();
        Polyline::<C>::new(a, None)
    }
}

impl<C: Coord> From<geo_types::LineString<f64>> for Polyline<C>
where
    C: From<geo_types::Coord<f64>>,
{
    fn from(line_string: geo_types::LineString<f64>) -> Self {
        let a = vec![line_string.into()];
        Polyline::<C>::new(a, None)
    }
}

impl<C: Coord> From<geo_types::Line<f64>> for Polyline<C>
where
    C: From<geo_types::Coord<f64>>,
{
    fn from(line_string: geo_types::Line<f64>) -> Self {
        let a = vec![LineString::<C>::new(vec![
            (line_string.start).into(),
            (line_string.end).into(),
        ])];
        Polyline::<C>::new(a, None)
    }
}
