use esri_json::geo_types_n::CoordNumber;
use esri_json::geometry::*;

pub fn default_point<C>() -> Point<C>
where
    C: Coord,
    C::T: CoordNumber + From<f32>,
{
    Point::<C>::new(C::from_coord_fields(0.0, 0.0, Some(0.0), None), None)
}

pub fn default_multipoint<C>() -> MultiPoint<C>
where
    C: Coord,
    C::T: CoordNumber + From<f32>,
{
    MultiPoint::new(
        vec![
            C::from_coord_fields(0.0, 0.0, Some(0.0), None),
            C::from_coord_fields(1.0, 1.0, Some(0.0), None),
        ],
        None,
    )
}

fn default_linestring<C>() -> LineString<C>
where
    C: Coord,
    C::T: CoordNumber + From<f32>,
{
    LineString::new(vec![
        C::from_coord_fields(0.0, 0.0, Some(0.0), None),
        C::from_coord_fields(1.0, 1.0, Some(1.0), None),
    ])
}

pub fn default_polyline<C>() -> Polyline<C>
where
    C: Coord,
    C::T: CoordNumber + From<f32>,
{
    Polyline::new(vec![default_linestring::<C>()], None)
}

fn default_ring<C>() -> LineString<C>
where
    C: Coord,
    C::T: CoordNumber + From<f32>,
{
    LineString::new(vec![
        C::from_coord_fields(0.0, 0.0, None, None),
        C::from_coord_fields(1.0, 0.0, None, None),
        C::from_coord_fields(1.0, 1.0, None, None),
        C::from_coord_fields(0.0, 1.0, None, None),
        C::from_coord_fields(0.0, 0.0, None, None),
    ])
}

pub fn default_polygon<C>() -> Polygon<C>
where
    C: Coord,
    C::T: CoordNumber + From<f32>,
{
    Polygon::new(vec![default_ring::<C>()], None)
}
