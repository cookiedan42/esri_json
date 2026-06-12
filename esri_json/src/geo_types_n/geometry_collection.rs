use geo_traits::GeometryCollectionTrait;

use super::Coord;
use super::Geometry;

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct GeometryCollection<C: Coord>(pub Vec<Geometry<C>>);

impl<C: Coord> GeometryCollectionTrait for GeometryCollection<C> {
    type GeometryType<'a>
        = Geometry<C>
    where
        C: 'a;

    fn num_geometries(&self) -> usize {
        self.0.len()
    }
    unsafe fn geometry_unchecked(&self, i: usize) -> Self::GeometryType<'_> {
        self.0[i].clone()
    }
}

impl<C: Coord> From<&geo_types::GeometryCollection<C::T>> for GeometryCollection<C>
where
    C: From<geo_types::Coord<C::T>>,
    super::Point<C>: for<'a> From<&'a geo_types::Point<C::T>>,
    super::Line<C>: for<'a> From<&'a geo_types::Line<C::T>>,
    super::LineString<C>: for<'a> From<&'a geo_types::LineString<C::T>>,
    super::Polygon<C>: for<'a> From<&'a geo_types::Polygon<C::T>>,
    super::MultiPoint<C>: for<'a> From<&'a geo_types::MultiPoint<C::T>>,
    super::MultiLineString<C>: for<'a> From<&'a geo_types::MultiLineString<C::T>>,
    super::MultiPolygon<C>: for<'a> From<&'a geo_types::MultiPolygon<C::T>>,
    super::Rect<C>: for<'a> From<&'a geo_types::Rect<C::T>>,
    super::Triangle<C>: for<'a> From<&'a geo_types::Triangle<C::T>>,
{
    fn from(value: &geo_types::GeometryCollection<C::T>) -> Self {
        GeometryCollection(value.iter().map(|g| convert_geometry(g)).collect())
    }
}

// Helper function to convert without triggering the recursive bound
fn convert_geometry<C: Coord>(value: &geo_types::Geometry<C::T>) -> Geometry<C>
where
    C: From<geo_types::Coord<C::T>>,
    super::Point<C>: for<'a> From<&'a geo_types::Point<C::T>>,
    super::Line<C>: for<'a> From<&'a geo_types::Line<C::T>>,
    super::LineString<C>: for<'a> From<&'a geo_types::LineString<C::T>>,
    super::Polygon<C>: for<'a> From<&'a geo_types::Polygon<C::T>>,
    super::MultiPoint<C>: for<'a> From<&'a geo_types::MultiPoint<C::T>>,
    super::MultiLineString<C>: for<'a> From<&'a geo_types::MultiLineString<C::T>>,
    super::MultiPolygon<C>: for<'a> From<&'a geo_types::MultiPolygon<C::T>>,
    super::Rect<C>: for<'a> From<&'a geo_types::Rect<C::T>>,
    super::Triangle<C>: for<'a> From<&'a geo_types::Triangle<C::T>>,
{
    match value {
        geo_types::Geometry::Point(p) => Geometry::Point(p.into()),
        geo_types::Geometry::Line(p) => Geometry::Line(p.into()),
        geo_types::Geometry::LineString(p) => Geometry::LineString(p.into()),
        geo_types::Geometry::Polygon(p) => Geometry::Polygon(p.into()),
        geo_types::Geometry::MultiPoint(p) => Geometry::MultiPoint(p.into()),
        geo_types::Geometry::MultiLineString(p) => Geometry::MultiLineString(p.into()),
        geo_types::Geometry::MultiPolygon(p) => Geometry::MultiPolygon(p.into()),
        geo_types::Geometry::GeometryCollection(gc) => Geometry::GeometryCollection(
            GeometryCollection(gc.iter().map(|g| convert_geometry(g)).collect()),
        ),
        geo_types::Geometry::Rect(p) => Geometry::Rect(p.into()),
        geo_types::Geometry::Triangle(p) => Geometry::Triangle(p.into()),
    }
}
