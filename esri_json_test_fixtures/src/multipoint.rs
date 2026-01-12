pub fn xy() -> String {
    r#"{
  "points":
  [
    [-97.06138, 32.837],
    [-97.06133, 33.836],
    [-98.2, 34.834],
    [-97, 40]
  ],
  "spatialReference": { "wkid": 4326 }
}"#
    .to_string()
}

pub fn xyz() -> String {
    r#"{
  "hasZ": true,
  "points":
  [
    [ -97.06138, 32.837, 35.2],
    [ -97.06133, 32.836, 35.3],
    [-97.06124, 32.834, 35.4],
    [-97.06127, 32.832, 35.5]
  ]
}"#
    .to_string()
}

pub fn xym() -> String {
    r#"{
  "hasM": true,
  "points":
  [
    [-97.06138, 32.837],
    [-97.06133, 33.836],
    [-98.2, 34.834],
    [-97, 40]
  ]
}"#
    .to_string()
}

pub fn xymi() -> String {
    r#"{
  "hasM": true,
  "points":
  [
    [-97.06138, 32.837],
    [-97.06133, 33.836],
    [-98.2, 34.834],
    [-97, 40]
  ],
  "ids": [1, 2, 3, 4]
}"#
    .to_string()
}

pub fn empty() -> String {
    r#"{
  "points": []
}"#
    .to_string()
}
