use std::iter::once;

use geo::winding_order::WindingOrder;
use geo::{Covers, Intersects, Winding};

use crate::geo_types_n;
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

// do xy math and map results back onto the original data

impl<C: Coord> From<Polygon<C>> for geo_types_n::MultiPolygon<C>
where
    geo_types::LineString<C::T>: for<'a> From<&'a geo_types_n::LineString<C>>,
    geo_types::Polygon<C::T>: for<'a> From<&'a geo_types_n::Polygon<C>>,
{
    fn from(val: Polygon<C>) -> Self {
        if val.rings().len() == 1 {
            let ring: geo_types_n::LineString<C> = val.rings()[0].clone().into();
            let ring = clone_to_winding_order(&ring, WindingOrder::CounterClockwise);
            return Self(vec![geo_types_n::Polygon::new(ring, vec![])]);
        }

        // otherwise, decompose into rings and validate

        // if only one ring, shortut
        let (exteriors, interiors): (Vec<_>, Vec<_>) = val
            .into_iter()
            .map(Into::<geo_types_n::LineString<C>>::into)
            .partition(|ring| is_ccw(ring));

        if exteriors.len() == 1 {
            let exterior = clone_to_winding_order(&exteriors[0], WindingOrder::CounterClockwise);
            let interiors = interiors
                .iter()
                .map(|ring| clone_to_winding_order(ring, WindingOrder::Clockwise));
            return Self(vec![geo_types_n::Polygon::new(
                exterior,
                interiors.collect(),
            )]);
        }

        // otherwise we are a MultiPolygon and need to associate holes with the exteriors
        let components = exteriors.into_iter().map(|e| make_polygon(e, &interiors));
        geo_types_n::MultiPolygon(components.collect())
    }
}

impl<C: Coord> From<geo_types_n::MultiPolygon<C>> for Polygon<C>
where
    geo_types::LineString<C::T>: for<'a> From<&'a geo_types_n::LineString<C>>,
{
    fn from(value: geo_types_n::MultiPolygon<C>) -> Self {
        let a = value
            .0
            .into_iter()
            .flat_map(|poly| {
                once(clone_to_winding_order::<C>(
                    poly.exterior(),
                    WindingOrder::Clockwise,
                ))
                .chain(
                    poly.interiors()
                        .map(|ring| clone_to_winding_order(ring, WindingOrder::CounterClockwise)),
                )
                .collect::<Vec<_>>()
            })
            .map(Into::<LineString<C>>::into);
        Polygon::<C>::new(a.collect(), None)
    }
}

impl<C: Coord> From<geo_types_n::Polygon<C>> for Polygon<C>
where
    geo_types::LineString<C::T>: for<'a> From<&'a geo_types_n::LineString<C>>,
{
    fn from(value: geo_types_n::Polygon<C>) -> Self {
        let a = once(clone_to_winding_order(
            value.exterior(),
            WindingOrder::Clockwise,
        ))
        .chain(
            value
                .interiors()
                .map(|ring| clone_to_winding_order(ring, WindingOrder::CounterClockwise)),
        )
        .map(Into::<LineString<C>>::into);
        Polygon::<C>::new(a.collect(), None)
    }
}

impl<C: Coord> From<geo_types_n::Rect<C>> for Polygon<C>
where
    geo_types::LineString<C::T>: for<'a> From<&'a geo_types_n::LineString<C>>,
{
    fn from(value: geo_types_n::Rect<C>) -> Self {
        value.to_polygon().into()
    }
}

impl<C: Coord> From<geo_types_n::Triangle<C>> for Polygon<C>
where
    geo_types::LineString<C::T>: for<'a> From<&'a geo_types_n::LineString<C>>,
{
    fn from(value: geo_types_n::Triangle<C>) -> Self {
        // polygon's into handles ring winding for us
        let ring = geo_types_n::LineString(vec![value.0, value.1, value.2, value.0]);
        geo_types_n::Polygon::new(ring, vec![]).into()
    }
}

// --- Helper Functions ---

// given a known exterior and potential holes, construct the polygon
// ensures that holes fully within other holes are removed
// ensures holes around the exterior are removed
fn make_polygon<C: Coord>(
    exterior: geo_types_n::LineString<C>,
    holes: &[geo_types_n::LineString<C>],
) -> geo_types_n::Polygon<C>
where
    geo_types::LineString<C::T>: for<'a> From<&'a geo_types_n::LineString<C>>,
    geo_types::Polygon<C::T>: for<'a> From<&'a geo_types_n::Polygon<C>>,
{
    let ext = geo_types_n::Polygon::<C>::new(exterior.clone(), vec![]);

    let candidate_holes = holes
        .iter()
        .filter(|hole| intersects(&ext, hole))
        .collect::<Vec<_>>();
    // drop holes which are fully within other holes
    let candidate_holes_as_polygons = candidate_holes
        .iter()
        .map(|candidate| geo_types_n::Polygon::<C>::new((*candidate).clone(), vec![]))
        .collect::<Vec<_>>();

    let holes_in_ext = candidate_holes.iter().filter(|candidate| {
        candidate_holes_as_polygons
            .iter()
            .any(|poly| covers(poly, candidate))
    });

    let c = holes_in_ext
        .map(|&ls| clone_to_winding_order(ls, WindingOrder::Clockwise))
        .collect::<Vec<_>>();
    let exterior = clone_to_winding_order(&exterior, WindingOrder::CounterClockwise);

    geo_types_n::Polygon::<C>::new(exterior, c)
}

fn is_ccw<C: Coord>(ring: &geo_types_n::LineString<C>) -> bool
where
    geo_types::LineString<C::T>: for<'a> From<&'a geo_types_n::LineString<C>>,
{
    let r: geo_types::LineString<C::T> = ring.into();
    Winding::is_ccw(&r)
}

fn intersects<C: Coord>(
    exterior: &geo_types_n::Polygon<C>,
    hole: &geo_types_n::LineString<C>,
) -> bool
where
    geo_types::Polygon<C::T>: for<'a> From<&'a geo_types_n::Polygon<C>>,
    geo_types::LineString<C::T>: for<'a> From<&'a geo_types_n::LineString<C>>,
{
    let p: geo_types::Polygon<C::T> = exterior.into();
    let h: geo_types::LineString<C::T> = hole.into();
    p.intersects(&h)
}

fn covers<C: Coord>(exterior: &geo_types_n::Polygon<C>, hole: &geo_types_n::LineString<C>) -> bool
where
    geo_types::Polygon<C::T>: for<'a> From<&'a geo_types_n::Polygon<C>>,
    geo_types::LineString<C::T>: for<'a> From<&'a geo_types_n::LineString<C>>,
{
    let p: geo_types::Polygon<C::T> = exterior.into();
    let h: geo_types::LineString<C::T> = hole.into();
    p.covers(&h)
}

fn clone_to_winding_order<C: Coord>(
    ring: &geo_types_n::LineString<C>,
    winding_order: WindingOrder,
) -> geo_types_n::LineString<C>
where
    geo_types::LineString<C::T>: for<'a> From<&'a geo_types_n::LineString<C>>,
{
    let r: geo_types::LineString<C::T> = ring.into();
    if r.winding_order() == Some(winding_order) {
        ring.clone()
    } else {
        let mut coords = ring.0.clone();
        coords.reverse();
        geo_types_n::LineString::<C>(coords)
    }
}
