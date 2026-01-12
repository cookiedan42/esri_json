import esri_json_py
import shapely


def test_pt():
    pt = shapely.Point([0, 0])
    assert (
        esri_json_py.wkt_to_esri(pt.wkt) == '{"spatialReference":null,"x":0.0,"y":0.0}'
    )


def test_cw_ccw():
    cw = shapely.Polygon([[0, 0], [1, 0], [1, 1], [0, 1], [0, 0]]).wkt
    ccw = shapely.Polygon([[0, 0], [0, 1], [1, 1], [1, 0], [0, 0]]).wkt

    assert cw != ccw

    assert esri_json_py.wkt_to_esri(cw) == esri_json_py.wkt_to_esri(ccw)


def test_geojson():
    geojson = shapely.Polygon([[0, 0], [1, 0], [1, 1], [0, 1], [0, 0]])

    assert esri_json_py.wkt_to_esri(geojson.wkt) == esri_json_py.geojson_to_esri(
        shapely.to_geojson(geojson)
    )
