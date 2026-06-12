use super::Coord;
use super::{
    GeometryCollection, Line, LineString, MultiLineString, MultiPoint, MultiPolygon, Point,
    Polygon, Rect, Triangle,
};
use crate::CoordNumber;
use geo_traits::CoordTrait;
use geo_traits::GeometryTrait;

#[derive(Eq, PartialEq, Clone, Hash)]
pub enum Geometry<C: Coord> {
    Point(Point<C>),
    Line(Line<C>),
    LineString(LineString<C>),
    Polygon(Polygon<C>),
    MultiPoint(MultiPoint<C>),
    MultiLineString(MultiLineString<C>),
    MultiPolygon(MultiPolygon<C>),
    GeometryCollection(GeometryCollection<C>),
    Rect(Rect<C>),
    Triangle(Triangle<C>),
}

impl<C: Coord> GeometryTrait for Geometry<C>
where
    <C as CoordTrait>::T: CoordNumber,
{
    type T = <C as CoordTrait>::T;
    type PointType<'a>
        = Point<C>
    where
        C: 'a;
    type LineStringType<'a>
        = LineString<C>
    where
        C: 'a;
    type PolygonType<'a>
        = Polygon<C>
    where
        C: 'a;
    type MultiPointType<'a>
        = MultiPoint<C>
    where
        C: 'a;
    type MultiLineStringType<'a>
        = MultiLineString<C>
    where
        C: 'a;
    type MultiPolygonType<'a>
        = MultiPolygon<C>
    where
        C: 'a;
    type GeometryCollectionType<'a>
        = GeometryCollection<C>
    where
        C: 'a;
    type RectType<'a>
        = Rect<C>
    where
        C: 'a;
    type TriangleType<'a>
        = Triangle<C>
    where
        C: 'a;
    type LineType<'a>
        = Line<C>
    where
        C: 'a;

    fn dim(&self) -> geo_traits::Dimensions {
        <C as Coord>::dim()
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        match self {
            Geometry::Point(p) => geo_traits::GeometryType::Point(p),
            Geometry::Line(p) => geo_traits::GeometryType::Line(p),
            Geometry::LineString(p) => geo_traits::GeometryType::LineString(p),
            Geometry::Polygon(p) => geo_traits::GeometryType::Polygon(p),
            Geometry::MultiPoint(p) => geo_traits::GeometryType::MultiPoint(p),
            Geometry::MultiLineString(p) => geo_traits::GeometryType::MultiLineString(p),
            Geometry::MultiPolygon(p) => geo_traits::GeometryType::MultiPolygon(p),
            Geometry::GeometryCollection(p) => geo_traits::GeometryType::GeometryCollection(p),
            Geometry::Rect(p) => geo_traits::GeometryType::Rect(p),
            Geometry::Triangle(p) => geo_traits::GeometryType::Triangle(p),
        }
    }
}

macro_rules! impl_geometrytrait_specialization {
    ($geometry_type:ident, $variant:expr) => {
        impl<C: Coord> GeometryTrait for $geometry_type<C>
        where
            <C as CoordTrait>::T: CoordNumber,
        {
            type T = <C as CoordTrait>::T;
            type PointType<'a>
                = Point<C>
            where
                C: 'a;
            type LineStringType<'a>
                = LineString<C>
            where
                C: 'a;
            type PolygonType<'a>
                = Polygon<C>
            where
                C: 'a;
            type MultiPointType<'a>
                = MultiPoint<C>
            where
                C: 'a;
            type MultiLineStringType<'a>
                = MultiLineString<C>
            where
                C: 'a;
            type MultiPolygonType<'a>
                = MultiPolygon<C>
            where
                C: 'a;
            type GeometryCollectionType<'a>
                = GeometryCollection<C>
            where
                C: 'a;
            type RectType<'a>
                = Rect<C>
            where
                C: 'a;
            type TriangleType<'a>
                = Triangle<C>
            where
                C: 'a;
            type LineType<'a>
                = Line<C>
            where
                C: 'a;

            fn dim(&self) -> geo_traits::Dimensions {
                <C as Coord>::dim()
            }

            fn as_type(
                &self,
            ) -> geo_traits::GeometryType<
                '_,
                Self::PointType<'_>,
                Self::LineStringType<'_>,
                Self::PolygonType<'_>,
                Self::MultiPointType<'_>,
                Self::MultiLineStringType<'_>,
                Self::MultiPolygonType<'_>,
                Self::GeometryCollectionType<'_>,
                Self::RectType<'_>,
                Self::TriangleType<'_>,
                Self::LineType<'_>,
            > {
                $variant(self)
            }
        }
    };
}

impl_geometrytrait_specialization!(Point, geo_traits::GeometryType::Point);
impl_geometrytrait_specialization!(MultiPoint, geo_traits::GeometryType::MultiPoint);

impl_geometrytrait_specialization!(Line, geo_traits::GeometryType::Line);
impl_geometrytrait_specialization!(LineString, geo_traits::GeometryType::LineString);
impl_geometrytrait_specialization!(MultiLineString, geo_traits::GeometryType::MultiLineString);

impl_geometrytrait_specialization!(Triangle, geo_traits::GeometryType::Triangle);
impl_geometrytrait_specialization!(Rect, geo_traits::GeometryType::Rect);

impl_geometrytrait_specialization!(Polygon, geo_traits::GeometryType::Polygon);
impl_geometrytrait_specialization!(MultiPolygon, geo_traits::GeometryType::MultiPolygon);

impl_geometrytrait_specialization!(
    GeometryCollection,
    geo_traits::GeometryType::GeometryCollection
);

impl<C: Coord> From<&Geometry<C>> for geo_types::Geometry<C::T>
where
    geo_types::Coord<C::T>: for<'a> From<&'a C>,
    geo_types::Point<C::T>: for<'a> From<&'a Point<C>>,
    geo_types::Line<C::T>: for<'a> From<&'a Line<C>>,
    geo_types::LineString<C::T>: for<'a> From<&'a LineString<C>>,
    geo_types::Polygon<C::T>: for<'a> From<&'a Polygon<C>>,
    geo_types::MultiPoint<C::T>: for<'a> From<&'a MultiPoint<C>>,
    geo_types::MultiLineString<C::T>: for<'a> From<&'a MultiLineString<C>>,
    geo_types::MultiPolygon<C::T>: for<'a> From<&'a MultiPolygon<C>>,
    geo_types::GeometryCollection<C::T>: for<'a> From<&'a GeometryCollection<C>>,
    geo_types::Rect<C::T>: for<'a> From<&'a Rect<C>>,
    geo_types::Triangle<C::T>: for<'a> From<&'a Triangle<C>>,
{
    fn from(value: &Geometry<C>) -> Self {
        match value {
            Geometry::Point(p) => geo_types::Geometry::Point(p.into()),
            Geometry::Line(p) => geo_types::Geometry::Line(p.into()),
            Geometry::LineString(p) => geo_types::Geometry::LineString(p.into()),
            Geometry::Polygon(p) => geo_types::Geometry::Polygon(p.into()),
            Geometry::MultiPoint(p) => geo_types::Geometry::MultiPoint(p.into()),
            Geometry::MultiLineString(p) => geo_types::Geometry::MultiLineString(p.into()),
            Geometry::MultiPolygon(p) => geo_types::Geometry::MultiPolygon(p.into()),
            Geometry::GeometryCollection(p) => geo_types::Geometry::GeometryCollection(p.into()),
            Geometry::Rect(p) => geo_types::Geometry::Rect(p.into()),
            Geometry::Triangle(p) => geo_types::Geometry::Triangle(p.into()),
        }
    }
}
impl<C: Coord> From<&geo_types::Geometry<C::T>> for Geometry<C>
where
    C: From<geo_types::Coord<C::T>>,
    Point<C>: for<'a> From<&'a geo_types::Point<C::T>>,
    Line<C>: for<'a> From<&'a geo_types::Line<C::T>>,
    LineString<C>: for<'a> From<&'a geo_types::LineString<C::T>>,
    Polygon<C>: for<'a> From<&'a geo_types::Polygon<C::T>>,
    MultiPoint<C>: for<'a> From<&'a geo_types::MultiPoint<C::T>>,
    MultiLineString<C>: for<'a> From<&'a geo_types::MultiLineString<C::T>>,
    MultiPolygon<C>: for<'a> From<&'a geo_types::MultiPolygon<C::T>>,
    GeometryCollection<C>: for<'a> From<&'a geo_types::GeometryCollection<C::T>>,
    Rect<C>: for<'a> From<&'a geo_types::Rect<C::T>>,
    Triangle<C>: for<'a> From<&'a geo_types::Triangle<C::T>>,
{
    fn from(value: &geo_types::Geometry<C::T>) -> Self {
        match value {
            geo_types::Geometry::Point(p) => Geometry::Point(p.into()),
            geo_types::Geometry::Line(p) => Geometry::Line(p.into()),
            geo_types::Geometry::LineString(p) => Geometry::LineString(p.into()),
            geo_types::Geometry::Polygon(p) => Geometry::Polygon(p.into()),
            geo_types::Geometry::MultiPoint(p) => Geometry::MultiPoint(p.into()),
            geo_types::Geometry::MultiLineString(p) => Geometry::MultiLineString(p.into()),
            geo_types::Geometry::MultiPolygon(p) => Geometry::MultiPolygon(p.into()),
            geo_types::Geometry::GeometryCollection(p) => Geometry::GeometryCollection(p.into()),
            geo_types::Geometry::Rect(p) => Geometry::Rect(p.into()),
            geo_types::Geometry::Triangle(p) => Geometry::Triangle(p.into()),
        }
    }
}
impl_from!(geo_types::Geometry<C::T>, Geometry<C>);
impl_from!(Geometry<C>, geo_types::Geometry<C::T>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::CoordXy;

    #[test]
    fn test_from_geometry() {
        let source = geo::Geometry::Point(geo::Point::new(0.0, 0.0));
        let _target: Geometry<CoordXy<f64>> = source.into();
    }
}
