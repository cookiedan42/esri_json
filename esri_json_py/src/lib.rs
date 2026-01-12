use esri_json::geometry::CoordXy;
use geojson::GeoJson;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use wkt::TryFromWkt;

/// Convert a wkt string into it's corresponding esri_json representation
#[pyfunction]
fn wkt_to_esri(wkt_str: &str) -> PyResult<String> {
    let geom: geo_types::Geometry<f64> = geo_types::Geometry::try_from_wkt_str(wkt_str)
        .map_err(|e: wkt::conversion::Error| PyErr::new::<PyValueError, _>(e.to_string()))?;

    let esri_geom: esri_json::geometry::Geometry<CoordXy> = geom
        .try_into()
        .map_err(|e: String| PyErr::new::<PyValueError, _>(e.to_string()))?;

    serde_json::to_string(&esri_geom).map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))
}

#[pyfunction]
fn geojson_to_esri(geojson_feature_str: &str) -> PyResult<String> {
    let geojson = geojson_feature_str
        .parse::<GeoJson>()
        .map_err(|e: geojson::Error| PyErr::new::<PyValueError, _>(e.to_string()))?;

    let (geom, _properties) = match geojson {
        GeoJson::Feature(f) => (f.geometry, f.properties),
        GeoJson::FeatureCollection(_f) => {
            return Err(PyErr::new::<PyValueError, _>(
                "FeatureCollection cannot be converted".to_string(),
            ));
        }
        GeoJson::Geometry(g) => (Some(g), None),
    };

    let geom: geo_types::Geometry<f64> = match geom {
        Some(g) => g
            .try_into()
            .map_err(|e: geojson::Error| PyErr::new::<PyValueError, _>(e.to_string()))?,
        None => return Err(PyErr::new::<PyValueError, _>("Empty geometry".to_string())),
    };

    let esri_geom: esri_json::geometry::Geometry<CoordXy> = geom
        .try_into()
        .map_err(|e: String| PyErr::new::<PyValueError, _>(e.to_string()))?;

    serde_json::to_string(&esri_geom).map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))
}

/// A Python module implemented in Rust.
#[pymodule]
fn esri_json_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(wkt_to_esri, m)?)?;
    m.add_function(wrap_pyfunction!(geojson_to_esri, m)?)?;
    Ok(())
}
