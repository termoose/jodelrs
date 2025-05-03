use serde::{Deserialize, Serialize};

pub struct Request {
    token: Option<String>,
}

#[derive(Debug, Serialize)]
struct Coordinates {
    lat: f32,
    lng: f32,
}

#[derive(Debug, Serialize)]
struct Location {
    city: String,
    country: String,
    loc_accuracy: String,
    loc_coordinates: Coordinates,
}

#[derive(Debug, Serialize)]
struct AccessTokenBody {
    language: String,
    client_id: String,
    device_uid: String,
    firebase_uid: String,
    #[serde(rename = "firebaseJWT")]
    firebase_jwt: String,
    location: Location,
}

impl Request {
    pub fn new() -> Self {
        Request { token: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // fn test_url_query() {
    //     let mut query = [
    //         ("channels", "true"),
    //         ("home", "false"),
    //         ("lat", "59.91"),
    //         ("lng", "10.77"),
    //         ("skipHometown", "false"),
    //         ("stickies", "true"),
    //     ];
    //     query.sort();

    //     let url = Url::parse_with_params("http://test.com", &query);
    // }
}
