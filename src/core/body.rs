use serde_json::Value;

pub trait ExpectBody {
    fn matches(&self, other: Value) -> bool;
}

impl ExpectBody for String {
    fn matches(&self, other: Value) -> bool {
        let actual: Value = serde_json::from_str(self).unwrap();

        actual == other
    }
}

impl ExpectBody for &str {
    fn matches(&self, other: Value) -> bool {
        let actual: Value = serde_json::from_str(self).unwrap();

        actual == other
    }
}

impl ExpectBody for Value {
    fn matches(&self, other: Value) -> bool {
        self.to_owned() == other
    }
}
