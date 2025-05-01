use std::collections::BTreeMap;

pub struct QueryParams {
    // The `BTreeMap` ensures the query parameters are always sorted
    params: BTreeMap<String, String>,
}

impl QueryParams {
    pub fn empty() -> Self {
        QueryParams {
            params: BTreeMap::new(),
        }
    }

    pub fn new<const N: usize>(params: [(&str, &str); N]) -> Self {
        QueryParams {
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
    fn test_create_empty() {
        let q = QueryParams::empty();
        assert_eq!(q.encode("whatever", "something"), "");
    }

    #[test]
    fn test_encode() {
        let q = QueryParams::new([("first", "value1"), ("second", "value2")]);

        assert_eq!(q.encode("=", "%"), "first=value1%second=value2");
        assert_eq!(q.encode("%", "%"), "first%value1%second%value2");
    }

    #[test]
    fn test_add() {
        let mut q = QueryParams::new([("first", "value1")]);
        q.add("second", "value2");

        assert_eq!(q.encode("=", "%"), "first=value1%second=value2");
        assert_eq!(q.encode("%", "%"), "first%value1%second%value2");
    }
}
