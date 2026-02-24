use crate::geo_types_n;
use crate::geometry::{Coord, LineString, Polyline};
// The only valid way to convert an esri Polyline to geo_types is to make it a MultiLineString
// Subsequent decomposition to LineString can be done by user
// But we can safely convert any of the 3 line type geometries to an esri Polyline

impl<C: Coord> From<Polyline<C>> for geo_types_n::MultiLineString<C>
where
    geo_types_n::LineString<C>: for<'a> From<&'a LineString<C>>,
{
    fn from(value: Polyline<C>) -> Self {
        Self(
            value
                .paths()
                .iter()
                .map(Into::<geo_types_n::LineString<C>>::into)
                .collect::<Vec<_>>(),
        )
    }
}

impl<C: Coord> From<geo_types_n::MultiLineString<C>> for Polyline<C>
where
    LineString<C>: for<'a> From<&'a geo_types_n::LineString<C>>,
{
    fn from(value: geo_types_n::MultiLineString<C>) -> Self {
        Polyline::<C>::new(
            value
                .0
                .iter()
                .map(Into::<LineString<C>>::into)
                .collect::<Vec<_>>(),
            None,
        )
    }
}

impl<C: Coord> From<geo_types_n::LineString<C>> for Polyline<C>
where
    LineString<C>: for<'a> From<&'a geo_types_n::LineString<C>>,
{
    fn from(line_string: geo_types_n::LineString<C>) -> Self {
        Polyline::<C>::new(vec![line_string.into()], None)
    }
}

impl<C: Coord> From<geo_types_n::Line<C>> for Polyline<C> {
    fn from(line_string: geo_types_n::Line<C>) -> Self {
        let a = vec![LineString::<C>::new(vec![
            line_string.start,
            line_string.end,
        ])];
        Polyline::<C>::new(a, None)
    }
}
