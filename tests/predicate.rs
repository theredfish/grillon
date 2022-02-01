use regex::Regex;
use serde_json::{json, Value};
use std::fmt::Debug;

pub enum AssertKind {
    Eq,
    Ne,
}

pub trait BodyAssert: Debug {
    fn assert(&self, other: Option<&Value>, kind: AssertKind);
}

impl BodyAssert for &str {
    fn assert(&self, other: Option<&Value>, kind: AssertKind) {
        let expected: Option<Value> = serde_json::from_str(self).ok();

        match kind {
            AssertKind::Eq => assert_eq!(
                expected.as_ref(),
                other,
                "The json body doesn't match the expected one. Expected : {:#?}, Found : {:#?}",
                expected,
                other,
            ),
            AssertKind::Ne => assert_ne!(
                expected.as_ref(),
                other,
                "The json body shouldn't match the expected one. Expected : {:#?}, Found : {:#?}",
                expected,
                other,
            ),
        }
    }
}

impl BodyAssert for String {
    fn assert(&self, other: Option<&Value>, kind: AssertKind) {
        let expected: Option<Value> = serde_json::from_str(self).ok();

        match kind {
            AssertKind::Eq => assert_eq!(
                expected.as_ref(),
                other,
                "The json body doesn't match the expected one. Expected : {:#?}, Found : {:#?}",
                expected,
                other,
            ),
            AssertKind::Ne => assert_ne!(
                expected.as_ref(),
                other,
                "The json body shouldn't match the expected one. Expected : {:#?}, Found : {:#?}",
                expected,
                other,
            ),
        }
    }
}

impl BodyAssert for Regex {
    fn assert(&self, actual: Option<&Value>, kind: AssertKind) {
        let actual = actual
            .expect("Cannot evaluate the regex against a body equals to None")
            .to_string();

        match kind {
            AssertKind::Eq => assert!(self.is_match(&actual), "The body doesn't match the regex"),
            AssertKind::Ne => assert!(
                !self.is_match(&actual),
                "The body matches the regex, which wasn't expected"
            ),
        }
    }
}

#[derive(Debug)]
pub enum BodyPredicate<T>
where
    T: BodyAssert,
{
    Is(T),
    IsNot(T),
    // Maybe see for the need of a separate enum as we need to enforce a Regex there
    Matches(T),
    DoesNotMatch(T),
}

pub struct Assert {
    pub json: Option<Value>,
}

impl Assert {
    pub fn body(&self, body: BodyPredicate<impl BodyAssert>) {
        match body {
            BodyPredicate::Is(assertion) => assertion.assert(self.json.as_ref(), AssertKind::Eq),
            BodyPredicate::IsNot(assertion) => assertion.assert(self.json.as_ref(), AssertKind::Ne),
            BodyPredicate::Matches(assertion) => {
                assertion.assert(self.json.as_ref(), AssertKind::Eq)
            }
            BodyPredicate::DoesNotMatch(assertion) => {
                assertion.assert(self.json.as_ref(), AssertKind::Ne)
            }
        }
    }
}

#[tokio::test]
pub async fn should_be_valid() {
    let assert = Assert {
        json: Some(json!({"result": "great!!"})),
    };

    assert.body(BodyPredicate::Is(r#"{"result": "great!!"}"#));
    assert.body(BodyPredicate::IsNot(r#"{"result": "fine!!"}"#));
    assert.body(BodyPredicate::Matches(
        Regex::new("great!!").expect("Valid Regex"),
    ));
    assert.body(BodyPredicate::DoesNotMatch(
        Regex::new("fine!!").expect("Valid Regex"),
    ));
}

#[tokio::test]
#[should_panic]
pub async fn should_panic_is_not() {
    let assert = Assert {
        json: Some(json!({"result": "great!!"})),
    };

    assert.body(BodyPredicate::IsNot(r#"{"result": "great!!"}"#));
}

#[tokio::test]
#[should_panic]
pub async fn should_panic_does_not_match_str() {
    let assert = Assert {
        json: Some(json!({"result": "great!!"})),
    };

    // This is possible because of the generic type : should we keep it or not? Give some flexibility but isn't transparent.
    assert.body(BodyPredicate::DoesNotMatch(r#"{"result": "great!!"}"#));
}

#[tokio::test]
#[should_panic]
pub async fn should_panic_does_not_match_regex() {
    let assert = Assert {
        json: Some(json!({"result": "great!!"})),
    };

    assert.body(BodyPredicate::DoesNotMatch(
        Regex::new("great!!").expect("Valid Regex"),
    ));
}
