use crate::geometry::Coord;
use crate::geometry::Geometry;

impl<C> TryInto<Geometry<C>> for geo_types::Geometry<f64>
where
    C: Coord + From<geo::Coord>,
{
    type Error = String;
    fn try_into(self) -> Result<Geometry<C>, Self::Error> {
        match self {
            geo_types::Geometry::Point(p) => Ok(Geometry::Point(p.into())),

            geo_types::Geometry::MultiPoint(mp) => Ok(Geometry::MultiPoint(mp.into())),

            geo_types::Geometry::Line(l) => Ok(Geometry::Polyline(l.into())),
            geo_types::Geometry::LineString(l) => Ok(Geometry::Polyline(l.into())),
            geo_types::Geometry::MultiLineString(l) => Ok(Geometry::Polyline(l.into())),

            geo_types::Geometry::Rect(p) => Ok(Geometry::Polygon(p.into())),
            geo_types::Geometry::Triangle(p) => Ok(Geometry::Polygon(p.into())),
            geo_types::Geometry::Polygon(p) => Ok(Geometry::Polygon(p.into())),
            geo_types::Geometry::MultiPolygon(p) => Ok(Geometry::Polygon(p.into())),

            geo_types::Geometry::GeometryCollection(_gc) => {
                Err("GeometryCollection cannot be converted".to_string())
            }
        }
    }
}
