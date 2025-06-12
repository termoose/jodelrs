use crate::{
    constants::{self, API_PATH_V2, API_SERVER, CLIENT_TYPE},
    crypto, query,
};

use anyhow::Result;
use chrono::{FixedOffset, Utc};
use http::{HeaderMap, HeaderValue, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct TokenData {
    access_token: String,
    refresh_token: String,
    expiration_date: i64,
}

pub struct Request {
    client: reqwest::Client,
    token: Option<TokenData>,
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
    pub fn create(lat: f32, lng: f32) -> Self {
        AccessTokenBody {
            language: "de-DE".to_string(),
            client_id: constants::CLIENT_ID.to_string(),
            device_uid: constants::DEVICE_UID.to_string(),
            firebase_uid: constants::FIREBASE_UID.to_string(),
            firebase_jwt: constants::FIREBASE_JWT.to_string(),
            location: Location {
                city: "".to_string(),
                country: "DE".to_string(),
                loc_accuracy: 10.56,
                loc_coordinates: Coordinates { lat, lng },
            },
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl Request {
    pub fn new() -> Self {
        Request {
            client: reqwest::Client::new(),
            token: None,
        }
    }

    async fn refresh_token(&mut self) -> Result<TokenData> {
        let uri = API_SERVER.to_owned() + API_PATH_V2 + "users/";
        let body = AccessTokenBody::create(59.91, 10.79);
        let timestamp = Utc::now().fixed_offset();
        let signature = crypto::compute_signature(
            None,
            http::Method::POST,
            &uri,
            timestamp,
            query::Params::empty(),
            Some(&body.to_json()),
        )?;

        let mut headers = query::Params::empty();
        headers.add("Accept", "application/json");
        headers.add("X-Client-Type", CLIENT_TYPE);
        headers.add("X-Api-Version", "0.2");
        headers.add(
            "X-Timestamp",
            &timestamp.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        );
        headers.add("X-Authorization", &format!("HMAC {}", signature));
        headers.add("Content-Type", "application/json; charset=UTF-8");

        let response: TokenData = self
            .client
            .post(uri)
            .headers(headers.into())
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_token_refresh() {
        let mut req = Request::new();
        assert!(req.token.is_none());

        let body = req.refresh_token();
        eprintln!("body: {:?}", body.await.unwrap());
        assert!(req.token.is_some());
    }

    #[test]
    fn verify_token_req_body() {
        let body_str = AccessTokenBody::create(59.91, 10.79).to_json();
        let wants = r#"{"language":"de-DE","client_id":"81e8a76e-1e02-4d17-9ba0-8a7020261b26","device_uid":"735d74003db1ce22358056ffbc8c4bb01a6008dcdfc24cf441ff1881938bd5a6","firebase_uid":"D0OQWsXcD2OdgsRyKgkdXyiiPBh2","firebaseJWT":"eyJhbGciOiJSUzI1NiIsImtpZCI6IjNmOWEwNTBkYzRhZTgyOGMyODcxYzMyNTYzYzk5ZDUwMjc3ODRiZTUiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL3NlY3VyZXRva2VuLmdvb2dsZS5jb20vdGVsbG0tYW5kcm9pZCIsImF1ZCI6InRlbGxtLWFuZHJvaWQiLCJhdXRoX3RpbWUiOjE3NDYyOTc2MDQsInVzZXJfaWQiOiJEME9RV3NYY0QyT2Rnc1J5S2drZFh5aWlQQmgyIiwic3ViIjoiRDBPUVdzWGNEMk9kZ3NSeUtna2RYeWlpUEJoMiIsImlhdCI6MTc0NjI5NzYwNSwiZXhwIjoxNzQ2MzAxMjA1LCJlbWFpbCI6ImNvbGQubW9vbjEzNDVAYmlya2VkLmFsIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsImZpcmViYXNlIjp7ImlkZW50aXRpZXMiOnsiZW1haWwiOlsiY29sZC5tb29uMTM0NUBiaXJrZWQuYWwiXX0sInNpZ25faW5fcHJvdmlkZXIiOiJwYXNzd29yZCJ9fQ.K2xVLdARjSw1hSPHhI9Cqt9tvvRHoHxiKufVsnLB0fZSqi9wQTeJ2B8X6b2HlbtQ2YQosyNntzLDyHQgr8flH1tfckbnEIb_C9wO2YaGXTFCZKfpysMJLzv7TuO1nVhe0ORr4pfsCq6ICRUP7AnW-Jd7WdnCMaZWckLp1MIQoPegrim6nkIaiuzVG-nkwIITBb9_gBGDaOTSD5ALroWDDILBT1mxl8yI-PNVR429UsLyDYuOTeNHAhL84vmgYpW7oiSeBBDkXHmDixXxsbLN3q2fkfc7QWHyK7IiPv_ekIMVrtLreswkAs7h0uLeeXFF5x9edzbi3Sw1QzwnIgMEmg","location":{"city":"","country":"DE","loc_accuracy":10.56,"loc_coordinates":{"lat":59.91,"lng":10.79}}}"#;

        assert_eq!(wants, body_str);
    }
}
