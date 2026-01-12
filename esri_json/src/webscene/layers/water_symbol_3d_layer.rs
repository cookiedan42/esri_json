//! Representation of a [WaterSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/waterSymbol3DLayer/)
//!
//! Symbol Layer that describes a water appearance on surfaces in a SceneView.
//!
//! Includes re-exports of relevant types

use crate::common::marker_type;
use serde::{Deserialize, Serialize};

pub use crate::common::Color;

marker_type!(T, "Water");

/// Representation of a [WaterSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/waterSymbol3DLayer/)
///
/// Symbol Layer that describes a water appearance on surfaces in a SceneView.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct WaterSymbol3DLayer {
    #[serde(rename = "type")]
    _type: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "waterbodySize")]
    waterbody_size: Option<WaterbodySize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "waveDirection")]
    wave_direction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "waveStrength")]
    wave_strength: Option<WaveStrength>,
}

impl WaterSymbol3DLayer {
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Size of the waterbody the symbol layer represents. Applications will display waves that are appropriate for the chosen body of water, for example ocean versus marina versus swimming pool.
    pub fn waterbody_size(mut self, size: WaterbodySize) -> Self {
        self.waterbody_size = Some(size);
        self
    }

    /// Azimuthal bearing for direction of the waves. If ommitted, waves appear directionless. The value is interpreted as 'geographic' rotation, i.e. clockwise starting from north.
    pub fn wave_direction(mut self, direction: f64) -> Self {
        self.wave_direction = Some(direction);
        self
    }

    /// The magnitude of the waves displayed on the waterbody. Strings roughly follow the [Douglas sea scale](https://en.wikipedia.org/wiki/Douglas_sea_scale), currently limited to lower degrees.
    pub fn wave_strength(mut self, strength: WaveStrength) -> Self {
        self.wave_strength = Some(strength);
        self
    }
}

/// Size of the waterbody the symbol layer represents. Applications will display waves that are appropriate for the chosen body of water, for example ocean versus marina versus swimming pool.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WaterbodySize {
    Small,
    Medium,
    Large,
}

/// The magnitude of the waves displayed on the waterbody. Strings roughly follow the [Douglas sea scale](https://en.wikipedia.org/wiki/Douglas_sea_scale), currently limited to lower degrees.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WaveStrength {
    Calm,
    Rippled,
    Slight,
    Moderate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let water_layer = r#"
        {
            "type": "Water"
        }"#;

        let de: WaterSymbol3DLayer = serde_json::from_str(water_layer).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: WaterSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[test]
    fn example() {
        let water_layer = r#"
        {
            "type": "Water",
            "color": [
                89,
                117,
                163
            ],
            "waterbodySize": "large",
            "waveDirection": 180,
            "waveStrength": "calm"
        }"#;

        let de: WaterSymbol3DLayer = serde_json::from_str(water_layer).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: WaterSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
