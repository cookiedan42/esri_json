pub fn line_xyz() -> String {
    r#"{
  "hasZ": true,
  "spatialReference": {
    "wkid": 4326
  },
  "paths": [
    [
      [
        -0.178,
        51.48791,
        0
      ],
      [
        -0.178,
        51.48791,
        1000
      ]
    ]
  ]
}"#
    .to_string()
}

pub fn line_xy() -> String {
    r#"{
  "spatialReference": {
    "wkid": 4326
  },
  "paths": [
    [
      [
        -0.178,
        51.48791
      ],
      [
        -0.178,
        51.48791
      ]
    ]
  ]
}"#
    .to_string()
}

pub fn line_xym() -> String {
    r#"{
  "hasM": true,
  "spatialReference": {
    "wkid": 4326
  },
  "paths": [
    [
      [
        -0.178,
        51.48791,
        0
      ],
      [
        -0.178,
        51.48791,
        1000
      ]
    ]
  ]
}"#
    .to_string()
}

pub fn line_xym_mixed() -> String {
    r#"{
  "hasM": true,
  "spatialReference": {
    "wkid": 4326
  },
  "paths": [
    [
      [
        -97.06138,
        32.837,
        5
      ],
      [
        -97.06133,
        32.836,
        6
      ],
      [
        -97.06124,
        32.834,
        7
      ]
    ],
    [
      [
        -97.06326,
        32.759
      ],
      [
        -97.06298,
        32.755
      ]
    ]
  ]
}"#
    .to_string()
}

pub fn line_xyzm() -> String {
    r#"{
  "hasM": true,
  "hasZ": true,
  "spatialReference": {
    "wkid": 4326
  },
  "paths": [
    [
      [
        -0.178,
        51.48791,
        0,
        1
      ],
      [
        -0.178,
        51.48791,
        1000,
        1
      ]
    ]
  ]
}"#
    .to_string()
}

pub fn empty() -> String {
    r#"{
  "paths": []
}"#
    .to_string()
}
