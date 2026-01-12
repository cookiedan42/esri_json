use geo::{BooleanOps, Intersects, Orient, Winding, unary_union};

use crate::geometry::{Coord, LineString, Polygon};

// The only valid way to convert an esri  Polygon to geo_types is to make it a MultiPolygon
// Subsequent decomposition to Polygon can be done by user
// But we can safely convert any of the polygon type geometries to a an esri Polygon

/*
esri --> geo_types

for each ring, identify handedness
Exterior rings are oriented clockwise, while holes are oriented counterclockwise.

for each exterior ring, find holes that are inside the exterior ring
and are not in some other hole
group these into polygons
return array of polygons as result
*/

/*
assuming that all rings are non-intersecting
for each ring, identify handedness and tag it
for each ring, identify holes
*/
fn is_exterior_ring(ring: &geo_types::LineString<f64>) -> bool {
    ring.is_cw()
}

// given a known exterior and potential holes, construct the polygon
// ensures that holes fully within other holes are removed
// ensures holes around the exterior are removed
fn make_polygon(
    exterior: &geo_types::LineString<f64>,
    holes: &[geo_types::LineString<f64>],
) -> geo_types::Polygon<f64> {
    let ext = geo_types::Polygon::<f64>::new(exterior.clone(), vec![]);

    let candidates = holes
        .iter()
        .filter(|&hole| hole.intersects(&ext))
        .collect::<Vec<_>>();
    // all remaining holes are inside the exterior

    let candidate_polygons = candidates
        .iter()
        .map(|candidate| geo_types::Polygon::<f64>::new((*candidate).clone(), vec![]))
        .collect::<Vec<_>>();
    let k = unary_union(&candidate_polygons);
    let c = k
        .iter()
        .map(|poly| poly.exterior())
        .cloned()
        .collect::<Vec<_>>();
    // all holes are non-intersecting
    geo_types::Polygon::<f64>::new(exterior.clone(), c)
}

impl<N: Coord> From<&Polygon<N>> for geo_types::MultiPolygon<f64>
where
    N: Into<geo_types::Coord<f64>>,
    for<'a> &'a LineString<N>: Into<geo_types::LineString<f64>>,
{
    fn from(val: &Polygon<N>) -> Self {
        if val.rings().len() == 1 {
            let ring = (&val.rings()[0]).into();
            let p = geo_types::Polygon::<f64>::new(ring, vec![]);
            return geo_types::MultiPolygon::new(vec![p]);
        }

        let exteriors = val
            .rings()
            .iter()
            .map(Into::<geo_types::LineString<f64>>::into)
            .filter(is_exterior_ring);

        let interiors = val
            .rings()
            .iter()
            .map(Into::<geo_types::LineString<f64>>::into)
            .filter(|r| !is_exterior_ring(r))
            .collect::<Vec<_>>();

        let components = exteriors.map(|e| make_polygon(&e, &interiors));

        geo_types::MultiPolygon::from_iter(components)
    }
}

/*
geo_types --> esri

orient exterior ring to be clockwise
orient holes to be counterclockwise
concatenate all rings into a single array
return array of rings as result
*/

impl<N: Coord> From<&geo_types::MultiPolygon<f64>> for Polygon<N>
where
    for<'a> N: From<&'a geo::Coord>,
{
    fn from(polygon: &geo_types::MultiPolygon<f64>) -> Self {
        let a = polygon
            .iter()
            .flat_map(orient_reversed::<N>)
            .collect::<Vec<_>>();
        Polygon::<N>::new(a, None)
    }
}
impl<N: Coord> From<geo_types::MultiPolygon<f64>> for Polygon<N>
where
    for<'a> N: From<&'a geo::Coord>,
{
    fn from(polygon: geo_types::MultiPolygon<f64>) -> Self {
        (&polygon).into()
    }
}

impl<N: Coord> From<&geo_types::Polygon<f64>> for Polygon<N>
where
    for<'a> N: From<&'a geo::Coord>,
{
    fn from(polygon: &geo_types::Polygon<f64>) -> Self {
        Polygon::<N>::new(orient_reversed(polygon), None)
    }
}

impl<N: Coord> From<geo_types::Polygon<f64>> for Polygon<N>
where
    for<'a> N: From<&'a geo::Coord>,
{
    fn from(polygon: geo_types::Polygon<f64>) -> Self {
        (&polygon).into()
    }
}

impl<N: Coord> From<&geo_types::Rect<f64>> for Polygon<N>
where
    for<'a> N: From<&'a geo::Coord>,
{
    fn from(polygon: &geo_types::Rect<f64>) -> Self {
        polygon.to_polygon().into()
    }
}

impl<N: Coord> From<geo_types::Rect<f64>> for Polygon<N>
where
    for<'a> N: From<&'a geo::Coord>,
{
    fn from(polygon: geo_types::Rect<f64>) -> Self {
        (&polygon).into()
    }
}

impl<N: Coord> From<&geo_types::Triangle<f64>> for Polygon<N>
where
    for<'a> N: From<&'a geo::Coord>,
{
    fn from(polygon: &geo_types::Triangle<f64>) -> Self {
        polygon.to_polygon().into()
    }
}

impl<N: Coord> From<geo_types::Triangle<f64>> for Polygon<N>
where
    for<'a> N: From<&'a geo::Coord>,
{
    fn from(polygon: geo_types::Triangle<f64>) -> Self {
        (&polygon).into()
    }
}

/// Orient a polygon to be clockwise external rings and counterclockwise holes
/// Prepares for conversion to Esri geometry
fn orient_reversed<N>(polygon: &geo_types::Polygon<f64>) -> Vec<LineString<N>>
where
    for<'a> N: Coord + From<&'a geo::Coord>,
{
    let p = polygon.orient(geo::orient::Direction::Reversed);
    p.rings()
        .map(Into::<LineString<N>>::into)
        .collect::<Vec<_>>()
}
