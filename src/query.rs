use http::{HeaderMap, HeaderName, HeaderValue};
use std::{collections::BTreeMap, str::FromStr};

pub struct Params {
    // The `BTreeMap` ensures the query parameters are always sorted
    params: BTreeMap<String, String>,
}

impl Into<HeaderMap> for Params {
    fn into(self) -> HeaderMap {
        self.params
            .iter()
            .map(|(k, v)| {
                (
                    HeaderName::from_str(k).unwrap(),
                    HeaderValue::from_str(v).unwrap(),
                )
            })
            .collect()
    }
}

impl Params {
    pub fn empty() -> Self {
        Params {
            params: BTreeMap::new(),
        }
    }

    pub fn new<const N: usize>(params: [(&str, &str); N]) -> Self {
        Params {
            params: params
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.params.insert(key.to_string(), value.to_string());
    }

    pub fn encode(&self, equal: &str, separator: &str) -> String {
        let params: Vec<String> = self
            .params
            .iter()
            .map(|(k, v)| k.to_owned() + equal + v)
            .collect();

        params.join(separator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_query_param_list() {
        let q = Params::empty();
        assert_eq!(q.encode("whatever", "something"), "");
    }

    #[test]
    fn encode_in_different_ways() {
        let q = Params::new([("first", "value1"), ("second", "value2")]);

        assert_eq!(q.encode("=", "%"), "first=value1%second=value2");
        assert_eq!(q.encode("%", "%"), "first%value1%second%value2");
    }

    #[test]
    fn add_additional_query_params() {
        let mut q = Params::new([("first", "value1")]);
        q.add("second", "value2");

        assert_eq!(q.encode("=", "%"), "first=value1%second=value2");
        assert_eq!(q.encode("%", "%"), "first%value1%second%value2");
    }

    #[test]
    fn convert_to_headers() {
        let headers: HeaderMap = Params::new([("first", "value1"), ("second", "value2")]).into();
        assert_eq!(
            headers.get("first"),
            Some(&HeaderValue::from_static("value1"))
        );
        assert_eq!(
            headers.get("second"),
            Some(&HeaderValue::from_static("value2"))
        );
    }
}
