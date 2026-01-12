use esri_json::geometry::*;

pub fn default_point<N: Coord>() -> Point<N> {
    Point::new(N::from_coord_fields(0.0, 0.0, Some(0.0), None), None)
}

pub fn default_multipoint<N: Coord>() -> MultiPoint<N> {
    MultiPoint::new(
        vec![
            N::from_coord_fields(0.0, 0.0, Some(0.0), None),
            N::from_coord_fields(1.0, 1.0, Some(0.0), None),
        ],
        None,
    )
}

fn default_linestring<N: Coord>() -> LineString<N> {
    LineString::new(vec![
        N::from_coord_fields(0.0, 0.0, Some(0.0), None),
        N::from_coord_fields(1.0, 1.0, Some(1.0), None),
    ])
}

pub fn default_polyline<N: Coord>() -> Polyline<N> {
    Polyline::new(vec![default_linestring::<N>()], None)
}

fn default_ring<N: Coord>() -> LineString<N> {
    LineString::new(vec![
        N::from_coord_fields(0.0, 0.0, None, None),
        N::from_coord_fields(1.0, 0.0, None, None),
        N::from_coord_fields(1.0, 1.0, None, None),
        N::from_coord_fields(0.0, 1.0, None, None),
        N::from_coord_fields(0.0, 0.0, None, None),
    ])
}

pub fn default_polygon<N: Coord>() -> Polygon<N> {
    Polygon::new(vec![default_ring::<N>()], None)
}
