use geo::winding_order::WindingOrder;
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

// given a known exterior and potential holes, construct the polygon
// ensures that holes fully within other holes are removed
// ensures holes around the exterior are removed
fn make_polygon(
    exterior: geo_types::LineString<f64>,
    holes: &[geo_types::LineString<f64>],
) -> geo_types::Polygon<f64> {
    let ext = geo_types::Polygon::<f64>::new(exterior.clone(), vec![]);

    let candidate_polygons = holes
        .iter()
        .filter(|&hole| hole.intersects(&ext))
        .map(|candidate| geo_types::Polygon::<f64>::new((*candidate).clone(), vec![]))
        .collect::<Vec<_>>();
    let k = unary_union(&candidate_polygons);
    let c = k
        .into_iter()
        .map(|poly| {
            poly.exterior()
                .clone_to_winding_order(WindingOrder::CounterClockwise)
        })
        .collect::<Vec<_>>();
    geo_types::Polygon::<f64>::new(exterior, c)
}

impl<C: Coord> From<Polygon<C>> for geo_types::MultiPolygon<f64>
where
    geo_types::Coord<f64>: From<C>,
    geo_types::LineString<f64>: From<LineString<C>>,
{
    fn from(val: Polygon<C>) -> Self {
        if val.rings().len() == 1 {
            let ring = val
                .into_iter()
                .map(Into::<geo_types::LineString<f64>>::into)
                .next()
                .expect("Expected single ring");
            let p = geo_types::Polygon::<f64>::new(ring, vec![]);
            return geo_types::MultiPolygon::new(vec![p]);
        }

        let (exteriors, interiors): (Vec<_>, Vec<_>) = val
            .into_iter()
            .map(Into::<geo_types::LineString<f64>>::into)
            .partition(Winding::is_cw);

        let components = exteriors.into_iter().map(|e| make_polygon(e, &interiors));

        geo_types::MultiPolygon::from_iter(components)
    }
}

impl<C: Coord> From<geo_types::MultiPolygon<f64>> for Polygon<C>
where
    C: From<geo::Coord>,
{
    fn from(polygon: geo_types::MultiPolygon<f64>) -> Self {
        let a = polygon
            .into_iter()
            .flat_map(orient_reversed::<C>)
            .collect::<Vec<_>>();
        Polygon::<C>::new(a, None)
    }
}

impl<C: Coord> From<geo_types::Polygon<f64>> for Polygon<C>
where
    C: From<geo::Coord>,
{
    fn from(polygon: geo_types::Polygon<f64>) -> Self {
        Polygon::<C>::new(orient_reversed(polygon), None)
    }
}

impl<C: Coord> From<geo_types::Rect<f64>> for Polygon<C>
where
    C: From<geo::Coord>,
{
    fn from(polygon: geo_types::Rect<f64>) -> Self {
        polygon.to_polygon().into()
    }
}

impl<C: Coord> From<geo_types::Triangle<f64>> for Polygon<C>
where
    C: From<geo::Coord>,
{
    fn from(polygon: geo_types::Triangle<f64>) -> Self {
        polygon.to_polygon().into()
    }
}

/// Orient a polygon to be clockwise external rings and counterclockwise holes
/// Prepares for conversion to Esri geometry
fn orient_reversed<C>(polygon: geo_types::Polygon<f64>) -> Vec<LineString<C>>
where
    C: Coord + From<geo::Coord>,
{
    let p = polygon.orient(geo::orient::Direction::Reversed);
    p.rings()
        .cloned()
        .map(Into::<LineString<C>>::into)
        .collect::<Vec<_>>()
}
