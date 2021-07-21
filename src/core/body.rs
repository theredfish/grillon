use serde_json::Value;

pub trait ExpectBody {
    fn to_value(&self) -> Value;
}

impl ExpectBody for String {
    fn to_value(&self) -> Value {
        serde_json::from_str(self).unwrap()
    }
}

impl ExpectBody for &str {
    fn to_value(&self) -> Value {
        serde_json::from_str(self).unwrap()
    }
}

impl ExpectBody for Value {
    fn to_value(&self) -> Value {
        self.to_owned()
    }
}
