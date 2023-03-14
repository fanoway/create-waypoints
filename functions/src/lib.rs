use serde::{Deserialize, Serialize};
use worker::*;
mod utils;
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, JsonObject, JsonValue, Value};

#[derive(Debug, Deserialize, Serialize)]
struct JsonGpx {
    gpx: String,
    metadata: String,
    link: String,
    time: String,
    trk: JsonTrk,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonTrk {
    name: String,
    trkseg: JsonTrkSeg,
}
#[derive(Debug, Deserialize, Serialize)]
struct JsonTrkSeg {
    trkpt: String,
}

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    //  Logging
    log_request(&req);
    utils::set_panic_hook();

    //  Routing
    let router = Router::new();

    router
        .options_async("/", |req, _| async move {
            let mut response = match req.method() {
                Method::Post => Response::ok(""),
                Method::Options => Response::ok(""),
                _ => Response::error("Invalid method", 405),
            };
            response = match response {
                Ok(resp) => {
                    let mut cors = Cors::default();
                    cors = cors.with_origins(vec!["*"]);
                    cors = cors.with_methods(vec![Method::Post, Method::Options]);
                    cors = cors.with_allowed_headers(vec!["*"]);
                    match resp.with_cors(&cors) {
                        Ok(resp) => Ok(resp),
                        Err(err) => Err(err),
                    }
                }
                Err(error) => Err(error),
            };
            return response;
        })
        .post_async("/", |mut req, _| async move {
            let geo_json_lines: Result<GeoJson> = match req.json().await {
                Ok(json) => Ok(json),
                Err(_) => Err(Error::Json(("Invalid JSON body content".to_string(), 400))),
            };

            let geo_json_points: Result<GeoJson> = match geo_json_lines {
                Ok(geo_json_lines) => convert_lines_to_waypoints(geo_json_lines).await,
                Err(error) => Err(error),
            };

            let mut response = match geo_json_points {
                Ok(json) => Response::from_json(&json),
                Err(err) => Response::error(err.to_string(), 400),
            };

            response = match response {
                Ok(resp) => {
                    let mut cors = Cors::default();
                    cors = cors.with_origins(vec!["*"]);
                    cors = cors.with_methods(vec![Method::Post, Method::Options]);
                    cors = cors.with_allowed_headers(vec!["*"]);
                    match resp.with_cors(&cors) {
                        Ok(resp) => Ok(resp),
                        Err(err) => Err(err),
                    }
                }
                Err(error) => Err(error),
            };

            // let res = match response {
            //     Ok(mut resp) => {
            //         match resp
            //             .headers_mut()
            //             .append("Access-Control-Allow-Origin", "*")
            //         {
            //             Ok(_) => (),
            //             Err(_) => (),
            //         };
            //         Ok(resp)
            //     }
            //     Err(error) => Err(error),
            // };

            // let res2 = match res {
            //     Ok(mut resp) => {
            //         match resp
            //             .headers_mut()
            //             .append("Access-Control-Allow-Methods", "POST")
            //         {
            //             Ok(_) => (),
            //             Err(_) => (),
            //         };
            //         Ok(resp)
            //     }
            //     Err(error) => Err(error),
            // };
            return response;
        })
        .run(req, env)
        .await
}

async fn convert_lines_to_waypoints(json_lines: GeoJson) -> Result<GeoJson> {
    // extract corner points of polygons
    let features = match FeatureCollection::try_from(json_lines) {
        Ok(collection) => collection.features,
        Err(_) => return Err(Error::Json(("Invalid JSON body content".to_string(), 400))),
    };

    let mut waypoints: Vec<Vec<f64>> = Vec::new();

    for feature in features {
        if let Value::Polygon(polys) = feature.geometry.unwrap().value {
            for shape_coords in polys {
                for coord in shape_coords {
                    waypoints.push(coord);
                }
            }
        }
    }

    //  Reconstruct geo json with just corner points
    let mut waypoint_features: Vec<Feature> = Vec::new();

    let mut corner_count = 1;

    for point in waypoints {
        let new_geom = Geometry::new(Value::Point(point));
        let mut properties = JsonObject::new();
        let key = "title".to_string();
        properties.insert(key, JsonValue::from(format!("Corner {}", &corner_count)));
        waypoint_features.push(Feature {
            bbox: None,
            geometry: Some(new_geom),
            id: None,
            properties: Some(properties),
            foreign_members: None,
        });
        corner_count += 1;
    }

    let json_points = GeoJson::FeatureCollection(FeatureCollection {
        bbox: None,
        features: waypoint_features,
        foreign_members: None,
    });

    Ok(json_points)
}
