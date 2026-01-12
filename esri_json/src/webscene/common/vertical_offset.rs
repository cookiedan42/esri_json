//! Representation of a [VerticalOffset](https://developers.arcgis.com/web-scene-specification/objects/verticalOffset/)
//!
//! Shifts the symbol along the vertical world axis by a given length. The length is set in screen space units.

use serde::{Deserialize, Serialize};

/// Representation of a [VerticalOffset](https://developers.arcgis.com/web-scene-specification/objects/verticalOffset/)
///
/// Shifts the symbol along the vertical world axis by a given length. The length is set in screen space units.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct VerticalOffset {
    #[serde(skip_serializing_if = "Option::is_none")]
    screen_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_world_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_world_length: Option<f64>,
}

/// Builder pattern
impl VerticalOffset {
    /// Maximal screen length of lift in points.
    pub fn screen_length(mut self, screen_length: f64) -> Self {
        self.screen_length = Some(screen_length);
        self
    }
    /// The maximum vertical symbol lift in world units. It acts as an upper bound to avoid lift becoming too big.
    pub fn max_world_length(mut self, max_world_length: f64) -> Self {
        self.max_world_length = Some(max_world_length);
        self
    }
    /// The minimum vertical symbol lift in world units. It acts as a lower bound to avoid lift becoming too small.
    pub fn min_world_length(mut self, min_world_length: f64) -> Self {
        self.min_world_length = Some(min_world_length);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_offset_example() {
        let vertical_offset = r#"{
            "screenLength": 100,
            "maxWorldLength": 600,
            "minWorldLength": 10
        }"#;
        let de: VerticalOffset = serde_json::from_str(vertical_offset).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: VerticalOffset = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
