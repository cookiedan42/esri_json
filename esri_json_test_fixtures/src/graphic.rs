pub fn point() -> String {
    r#"{
  "aggregateGeometries": null,
  "geometry": {
    "spatialReference": {
      "wkid": 4326
    },
    "x": -49.97,
    "y": 41.73
  },
  "symbol": {
    "type": "esriSMS",
    "color": [
      226,
      119,
      40,
      255
    ],
    "angle": 0,
    "xoffset": 0,
    "yoffset": 0,
    "size": 12,
    "style": "esriSMSCircle",
    "outline": {
      "type": "esriSLS",
      "color": [
        255,
        255,
        255,
        255
      ],
      "width": 2,
      "style": "esriSLSSolid"
    }
  },
  "attributes": {},
  "popupTemplate": {
    "title": "hello{gfa}",
    "description":" yippie my GFA is {gfa} "
  }
}"#
    .to_string()
}

pub fn polyline() -> String {
    r#"{
  "aggregateGeometries": null,
  "geometry": {
    "spatialReference": {
      "wkid": 4326
    },
    "paths": [
      [
        [
          -111.3,
          52.68
        ],
        [
          -98,
          49.5
        ],
        [
          -93.94,
          29.89
        ]
      ]
    ]
  },
  "symbol": {
    "type": "esriSLS",
    "color": [
      226,
      119,
      40,
      255
    ],
    "width": 4,
    "style": "esriSLSSolid",
    "marker": {
      "placement": "begin",
      "type": "line-marker",
      "style": "arrow",
      "color": null
    }
  },
  "attributes": {
    "Name": "Keystone Pipeline",
    "Owner": "TransCanada",
    "Length": "3,456 km"
  },
  "popupTemplate": null
}"#
    .to_string()
}

pub fn polygon() -> String {
    r#"{
  "aggregateGeometries": null,
  "geometry": {
    "spatialReference": {
      "wkid": 4326
    },
    "rings": [
      [
        [
          -64.78,
          32.3
        ],
        [
          -66.07,
          18.45
        ],
        [
          -80.21,
          25.78
        ],
        [
          -64.78,
          32.3
        ]
      ]
    ]
  },
  "symbol": {
    "type": "esriSFS",
    "color": [
      227,
      139,
      79,
      204
    ],
    "outline": {
      "type": "esriSLS",
      "color": [
        255,
        255,
        255,
        255
      ],
      "width": 1,
      "style": "esriSLSSolid"
    },
    "style": "esriSFSSolid"
  },
  "attributes": {},
  "popupTemplate": null
}"#
    .to_string()
}
