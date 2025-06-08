use crate::query;

use chrono::{DateTime, FixedOffset};
use hex;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use url::{ParseError, Url};

const KEY: &str = "weFXFtsXOWhsRyBYQHCuxZhjkvElVPbdWqqdoBuk";

fn sha1hmac(data: &str) -> String {
    let mut mac = Hmac::<Sha1>::new_from_slice(KEY.as_bytes()).expect("invalid key length");
    mac.update(data.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}

pub fn compute_signature(
    token: Option<&str>,
    method: http::Method,
    uri: &str,
    timestamp: DateTime<FixedOffset>,
    params: query::Params,
    body: Option<&str>,
) -> Result<String, ParseError> {
    let parsed = Url::parse(uri)?;
    let query_params = params.encode("%", "%");

    Ok(sha1hmac(&format!(
        "{}%{}%443%{}%{}%%{}%{}%{}",
        method.as_str(),
        parsed.host_str().unwrap(),
        parsed.path(),
        token.unwrap_or(""),
        timestamp.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        query_params,
        body.unwrap_or("")
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_sha1hmac_output() {
        let input = "GET%api.jodelapis.com%443%/api/v3/user/recommendedChannels%25027287-ee291561-948ba51f-b6a6-4b11-a02d-ac7639650fe9%%2024-07-11T19:16:12.100Z%home%false%";
        let wants = "5c411f124d9bf438c3cb5e66a00c3d2aa1c66e87";
        let output = sha1hmac(input);

        assert_eq!(wants, output);
    }

    #[test]
    fn verify_signature_outut() {
        let token = "25027287-ee291561-948ba51f-b6a6-4b11-a02d-ac7639650fe9";
        let method = http::Method::GET;
        let url = "https://api.jodelapis.com/api/v3/posts/location/combo";
        let timestamp = "2024-07-12T09:23:07.308Z";
        let params = query::Params::new([
            ("channels", "true"),
            ("home", "false"),
            ("lat", "59.91"),
            ("lng", "10.77"),
            ("skipHometown", "false"),
            ("stickies", "true"),
        ]);

        let wants = "19c552f785e54408a6d1b84d81ba87171f2c3cb3";
        let signature = compute_signature(
            Some(token),
            method,
            url,
            DateTime::parse_from_rfc3339(timestamp).unwrap(),
            params,
            None,
        );

        assert_eq!(wants, signature.unwrap());
    }
}
