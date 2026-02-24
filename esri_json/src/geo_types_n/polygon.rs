use std::slice::Iter;

use super::Coord;
use super::LineString;
use geo_traits::PolygonTrait;

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Polygon<C: Coord> {
    exterior: LineString<C>,
    interiors: Vec<LineString<C>>,
}
impl<C: Coord> Polygon<C> {
    pub fn new(exterior: LineString<C>, interiors: Vec<LineString<C>>) -> Self {
        Self {
            exterior,
            interiors,
        }
    }
    pub fn exterior(&self) -> &LineString<C> {
        &self.exterior
    }
    pub fn interiors(&self) -> Iter<'_, LineString<C>> {
        self.interiors.iter()
    }
}

impl<ILS, C> From<&[ILS]> for Polygon<C>
where
    for<'a> &'a ILS: Into<LineString<C>>,
    C: Coord,
{
    fn from(rings: &[ILS]) -> Self {
        let mut rings = rings.iter().map(|r| (r).into());
        Self {
            exterior: rings.next().unwrap(),
            interiors: rings.collect(),
        }
    }
}

impl<C: Coord> PolygonTrait for Polygon<C> {
    type RingType<'a>
        = LineString<C>
    where
        C: 'a;

    fn exterior(&self) -> Option<LineString<C>> {
        Some(self.exterior.clone())
    }
    unsafe fn interior_unchecked(&self, i: usize) -> Self::RingType<'_> {
        self.interiors[i].clone()
    }
    fn num_interiors(&self) -> usize {
        self.interiors.len()
    }
}

impl<C: Coord> From<&Polygon<C>> for geo_types::Polygon<f64>
where
    geo_types::LineString<f64>: for<'a> From<&'a LineString<C>>,
{
    fn from(val: &Polygon<C>) -> Self {
        Self::new(
            (&val.exterior).into(),
            val.interiors
                .iter()
                .map(Into::<geo_types::LineString<f64>>::into)
                .collect(),
        )
    }
}
impl<C: Coord> From<&geo_types::Polygon<f64>> for Polygon<C>
where
    LineString<C>: for<'a> From<&'a geo_types::LineString<f64>>,
{
    fn from(val: &geo_types::Polygon<f64>) -> Self {
        Self::new(
            val.exterior().into(),
            val.interiors().iter().map(Into::into).collect(),
        )
    }
}
impl_from!(geo_types::Polygon<f64>, Polygon<C>);
impl_from!(Polygon<C>, geo_types::Polygon<f64>);
