#[cfg(feature = "diff")]
use pretty_assertions::assert_eq;
use serde_json::Value;

pub trait BodyMatch {
    fn matches(&self, other: Option<&Value>);
}

impl BodyMatch for String {
    fn matches(&self, actual: Option<&Value>) {
        let expected: Option<Value> = serde_json::from_str(self).ok();
        assert_eq!(
            expected.as_ref(),
            actual,
            "The json body doesn't match the expected one. Expected : {:#?}, Found : {:#?}",
            expected,
            actual,
        );
    }
}

impl BodyMatch for &str {
    fn matches(&self, actual: Option<&Value>) {
        let expected: Option<Value> = serde_json::from_str(self).ok();
        assert_eq!(
            expected.as_ref(),
            actual,
            "The json body doesn't match the expected one. Expected : {:#?}, Found : {:#?}",
            expected,
            actual,
        );
    }
}

impl BodyMatch for Value {
    fn matches(&self, actual: Option<&Value>) {
        let expected = Some(self);

        assert_eq!(
            expected, actual,
            "The json body doesn't match the expected one. Expected : {:#?}, Found : {:#?}",
            expected, actual,
        );
    }
}
