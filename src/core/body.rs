use serde_json::Value;

pub trait BodyMatch {
    fn matches(&self, other: Value) -> bool;
}

impl BodyMatch for String {
    fn matches(&self, other: Value) -> bool {
        let actual: Value = serde_json::from_str(self).unwrap();

        actual == other
    }
}

impl BodyMatch for &str {
    fn matches(&self, other: Value) -> bool {
        let actual: Value = serde_json::from_str(self).unwrap();

        actual == other
    }
}

impl BodyMatch for Value {
    fn matches(&self, other: Value) -> bool {
        self.to_owned() == other
    }
}
