use esri_json::geometry::*;

pub fn default_point<C: Coord>() -> Point<C> {
    Point::new(C::from_coord_fields(0.0, 0.0, Some(0.0), None), None)
}

pub fn default_multipoint<C: Coord>() -> MultiPoint<C> {
    MultiPoint::new(
        vec![
            C::from_coord_fields(0.0, 0.0, Some(0.0), None),
            C::from_coord_fields(1.0, 1.0, Some(0.0), None),
        ],
        None,
    )
}

fn default_linestring<C: Coord>() -> LineString<C> {
    LineString::new(vec![
        C::from_coord_fields(0.0, 0.0, Some(0.0), None),
        C::from_coord_fields(1.0, 1.0, Some(1.0), None),
    ])
}

pub fn default_polyline<C: Coord>() -> Polyline<C> {
    Polyline::new(vec![default_linestring::<C>()], None)
}

fn default_ring<C: Coord>() -> LineString<C> {
    LineString::new(vec![
        C::from_coord_fields(0.0, 0.0, None, None),
        C::from_coord_fields(1.0, 0.0, None, None),
        C::from_coord_fields(1.0, 1.0, None, None),
        C::from_coord_fields(0.0, 1.0, None, None),
        C::from_coord_fields(0.0, 0.0, None, None),
    ])
}

pub fn default_polygon<C: Coord>() -> Polygon<C> {
    Polygon::new(vec![default_ring::<C>()], None)
}
