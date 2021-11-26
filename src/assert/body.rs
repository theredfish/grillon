use serde_json::Value;

pub trait BodyMatch {
    fn matches(&self, other: Option<&Value>) -> bool;
}

impl BodyMatch for String {
    fn matches(&self, other: Option<&Value>) -> bool {
        let actual: Option<Value> = serde_json::from_str(self).ok();
        actual.as_ref() == other
    }
}

impl BodyMatch for &str {
    fn matches(&self, other: Option<&Value>) -> bool {
        let actual: Option<Value> = serde_json::from_str(self).ok();
        actual.as_ref() == other
    }
}

impl BodyMatch for Value {
    fn matches(&self, other: Option<&Value>) -> bool {
        Some(self) == other
    }
}
