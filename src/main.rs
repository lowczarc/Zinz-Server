#![deny(warnings)]
use chrono::prelude::*;
use chrono::Utc;
use redis::Commands;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use std::sync::Mutex;
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct LocationDataInput {
    latitude: f32,
    longitude: f32,
}

#[derive(Serialize, Deserialize)]
struct LocationData {
    latitude: f32,
    longitude: f32,
    time: DateTime<Utc>,
}

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let redis_connection = Arc::new(Mutex::new(client.get_connection().unwrap()));

    let get_redis_connection_mutex = redis_connection.clone();
    let get_location = warp::get()
        .and(warp::path("location"))
        .and(warp::path::param())
        .and(warp::path::end())
        .map(move |user: String| {
            if !user.chars().all(char::is_alphanumeric) || user.len() == 0 {
                return json!({
                    "error": "invalid_user_format",
                    "error_description": "A username can only contain alphanumeric characters"
                }).to_string();
            }

            let mut redis_connection = get_redis_connection_mutex.lock().unwrap();
            let location_data_result: Result<String, _> = redis_connection.get(format!("location/{}", user));

            if let Ok(location_data) = location_data_result {
                if let Ok(location) = serde_json::from_str::<LocationData>(&location_data) {
                    serde_json::to_string(&location).unwrap()
                } else {
                    json!({
                        "error": "invalid_location_format",
                        "error_description": format!("Record location found for \"{}\" is in an invalid format", user)
                    }).to_string()
                }
            } else {
                json!({
                    "error": "not_found",
                    "error_description": format!("Record location for \"{}\" not found", user)
                }).to_string()
            }
        });

    let post_redis_connection_mutex = redis_connection.clone();
    let post_location = warp::post()
        .and(warp::path("location"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::query::<LocationDataInput>())
        .map(move |user: String, new_location_input: LocationDataInput| {
            if !user.chars().all(char::is_alphanumeric) || user.len() == 0 {
                return json!({
                    "error": "invalid_user_format",
                    "error_description": "A username can only contain alphanumeric characters"
                }).to_string();
            }

            let mut redis_connection = post_redis_connection_mutex.lock().unwrap();

            let new_location = LocationData {
                latitude: new_location_input.latitude,
                longitude: new_location_input.longitude,
                time: Utc::now(),
            };

            redis_connection
                .set::<&str, String, ()>(
                    &format!("location/{}", user),
                    serde_json::to_string(&new_location).unwrap(),
                )
                .unwrap();

            println!(
                "{}: {} location updated [LAT: {}, LON: {}]",
                new_location.time, user, new_location.latitude, new_location.longitude
            );

            json!({ "success": true }).to_string()
        });

    let static_routes = warp::any().and(warp::fs::dir("public"));

    let routes = get_location.or(post_location).or(static_routes);

    warp::serve(routes).run(([127, 0, 0, 1], 3001)).await;
}
