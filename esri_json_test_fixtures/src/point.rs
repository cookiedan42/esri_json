pub fn point_xy() -> String {
    r#"{"spatialReference":{"wkid": 4326},"x": -0.178,"y": 51.48791}"#.to_string()
}
pub fn point_xyz() -> String {
    r#"{"spatialReference":{"wkid": 4326},"x": -0.178,"y": 51.48791,"z": 1010}"#.to_string()
}
pub fn point_xym() -> String {
    r#"{"spatialReference":{"wkid": 4326},"x": -0.178,"y": 51.48791,"m": 123}"#.to_string()
}
pub fn point_xyzm() -> String {
    r#"{"spatialReference":{"wkid": 4326},"x": -0.178,"y": 51.48791,"z": 1010,"m": 123}"#
        .to_string()
}
