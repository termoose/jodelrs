use crate::constants;
use serde::Serialize;

pub struct Request {
    client: reqwest::Client,
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
    loc_accuracy: f32,
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

impl AccessTokenBody {
    pub fn create(city: &str, country: &str, lat: f32, lng: f32) -> Self {
        AccessTokenBody {
            language: "de-DE".to_string(),
            client_id: constants::CLIENT_ID.to_string(),
            device_uid: constants::DEVICE_UID.to_string(),
            firebase_uid: constants::FIREBASE_UID.to_string(),
            firebase_jwt: constants::FIREBASE_JWT.to_string(),
            location: Location {
                city: city.to_string(),
                country: country.to_string(),
                loc_accuracy: 10.56,
                loc_coordinates: Coordinates { lat, lng },
            },
        }
    }
}

impl Request {
    pub fn new() -> Self {
        Request {
            client: reqwest::Client::new(),
            token: None,
        }
    }

    pub fn refresh_token() {}
}

#[cfg(test)]
mod tests {
    use super::*;
}
