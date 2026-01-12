use crate::geometry::{Coord, CoordXy, CoordXym, CoordXyz, CoordXyzm};

macro_rules! FromIntoGeoCoord {
    ($coordtype:ty) => {
        impl From<$coordtype> for geo_types::Coord<f64> {
            fn from(c: $coordtype) -> Self {
                Self { x: c.x(), y: c.y() }
            }
        }

        impl From<&$coordtype> for geo_types::Coord<f64> {
            fn from(c: &$coordtype) -> Self {
                Self { x: c.x(), y: c.y() }
            }
        }

        impl From<geo_types::Coord<f64>> for $coordtype {
            fn from(coord: geo_types::Coord<f64>) -> Self {
                <$coordtype>::from_coord_fields(coord.x, coord.y, None, None)
            }
        }
        impl From<&geo_types::Coord<f64>> for $coordtype {
            fn from(coord: &geo_types::Coord<f64>) -> Self {
                <$coordtype>::from_coord_fields(coord.x, coord.y, None, None)
            }
        }
    };
}

FromIntoGeoCoord!(CoordXy);
FromIntoGeoCoord!(CoordXyz);
FromIntoGeoCoord!(CoordXym);
FromIntoGeoCoord!(CoordXyzm);
