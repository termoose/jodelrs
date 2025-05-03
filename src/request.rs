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

    #[test]
    fn verify_token_req_body() {
        let body = AccessTokenBody::create("", "DE", 59.91, 10.79);
        let body_str = serde_json::to_string(&body).unwrap();
        let wants = r#"{"language":"de-DE","client_id":"81e8a76e-1e02-4d17-9ba0-8a7020261b26","device_uid":"735d74003db1ce22358056ffbc8c4bb01a6008dcdfc24cf441ff1881938bd5a6","firebase_uid":"D0OQWsXcD2OdgsRyKgkdXyiiPBh2","firebaseJWT":"eyJhbGciOiJSUzI1NiIsImtpZCI6IjNmOWEwNTBkYzRhZTgyOGMyODcxYzMyNTYzYzk5ZDUwMjc3ODRiZTUiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL3NlY3VyZXRva2VuLmdvb2dsZS5jb20vdGVsbG0tYW5kcm9pZCIsImF1ZCI6InRlbGxtLWFuZHJvaWQiLCJhdXRoX3RpbWUiOjE3NDYyOTc2MDQsInVzZXJfaWQiOiJEME9RV3NYY0QyT2Rnc1J5S2drZFh5aWlQQmgyIiwic3ViIjoiRDBPUVdzWGNEMk9kZ3NSeUtna2RYeWlpUEJoMiIsImlhdCI6MTc0NjI5NzYwNSwiZXhwIjoxNzQ2MzAxMjA1LCJlbWFpbCI6ImNvbGQubW9vbjEzNDVAYmlya2VkLmFsIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsImZpcmViYXNlIjp7ImlkZW50aXRpZXMiOnsiZW1haWwiOlsiY29sZC5tb29uMTM0NUBiaXJrZWQuYWwiXX0sInNpZ25faW5fcHJvdmlkZXIiOiJwYXNzd29yZCJ9fQ.K2xVLdARjSw1hSPHhI9Cqt9tvvRHoHxiKufVsnLB0fZSqi9wQTeJ2B8X6b2HlbtQ2YQosyNntzLDyHQgr8flH1tfckbnEIb_C9wO2YaGXTFCZKfpysMJLzv7TuO1nVhe0ORr4pfsCq6ICRUP7AnW-Jd7WdnCMaZWckLp1MIQoPegrim6nkIaiuzVG-nkwIITBb9_gBGDaOTSD5ALroWDDILBT1mxl8yI-PNVR429UsLyDYuOTeNHAhL84vmgYpW7oiSeBBDkXHmDixXxsbLN3q2fkfc7QWHyK7IiPv_ekIMVrtLreswkAs7h0uLeeXFF5x9edzbi3Sw1QzwnIgMEmg","location":{"city":"","country":"DE","loc_accuracy":10.56,"loc_coordinates":{"lat":59.91,"lng":10.79}}}"#;

        assert_eq!(wants, body_str);
    }
}
