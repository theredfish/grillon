use crate::{
    assertion::{
        traits::{Equality, JsonPath},
        Assertion, AssertionResult, Hand,
    },
    dsl::{expression::Predicate, JsonPathExpr, Part},
    LogSettings,
};
use jsonpath_lib as jsonpath;
use serde_json::{json, Value};

/// Http json body DSL to assert body of a response.
pub trait JsonBodyDsl<T> {
    /// Evaluates the json body assertion to run based on the [`Predicate`].
    fn eval(&self, actual: T, predicate: Predicate, log_settings: &LogSettings)
        -> Assertion<Value>;
}

impl JsonBodyDsl<Value> for Value {
    fn eval(
        &self,
        actual: Value,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Value> {
        match predicate {
            Predicate::Is => self.is(actual).assert(log_settings),
            Predicate::IsNot => self.is_not(actual).assert(log_settings),
            _ => unimplemented!("Invalid predicate for the json body DSL : {predicate}"),
        }
    }
}

impl JsonBodyDsl<Value> for &str {
    fn eval(
        &self,
        actual: Value,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Value> {
        match predicate {
            Predicate::Is => self.is(actual).assert(log_settings),
            Predicate::IsNot => self.is_not(actual).assert(log_settings),
            _ => unimplemented!("Invalid predicate for the json body DSL : {predicate}"),
        }
    }
}

impl JsonBodyDsl<Value> for String {
    fn eval(
        &self,
        actual: Value,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Value> {
        match predicate {
            Predicate::Is => self.is(actual).assert(log_settings),
            Predicate::IsNot => self.is_not(actual).assert(log_settings),
            _ => unimplemented!("Invalid predicate for the json body DSL : {predicate}"),
        }
    }
}

impl JsonBodyDsl<Value> for JsonPathExpr<Value> {
    fn eval(
        &self,
        actual: Value,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Value> {
        match predicate {
            Predicate::JsonPath => self
                .json_path(actual, log_settings.to_owned())
                .assert(log_settings),
            _ => unimplemented!(),
        }
    }
}

pub trait JsonBodyDslEquality<T>: JsonBodyDsl<T> {
    /// Asserts the json response body is strictly equals to the provided value.
    fn is(&self, actual: T) -> Assertion<Value>;
    /// Asserts the json response body is strictly not equals to the provided value.
    fn is_not(&self, actual: T) -> Assertion<Value>;
}

impl JsonBodyDslEquality<Value> for Value {
    fn is(&self, actual: Value) -> Assertion<Value> {
        actual.is_eq(self)
    }

    fn is_not(&self, actual: Value) -> Assertion<Value> {
        actual.is_ne(self)
    }
}

impl JsonBodyDslEquality<Value> for &str {
    fn is(&self, actual: Value) -> Assertion<Value> {
        actual.is_eq(*self)
    }

    fn is_not(&self, actual: Value) -> Assertion<Value> {
        actual.is_ne(*self)
    }
}

impl JsonBodyDslEquality<Value> for String {
    fn is(&self, actual: Value) -> Assertion<Value> {
        actual.is_eq(self)
    }

    fn is_not(&self, actual: Value) -> Assertion<Value> {
        actual.is_ne(self)
    }
}

pub trait JsonBodyDslJsonpath<T>: JsonBodyDsl<T> {
    fn json_path(&self, actual: T, log_settings: LogSettings) -> Assertion<Value>;
}

impl JsonBodyDslJsonpath<Value> for JsonPathExpr<Value> {
    fn json_path(&self, actual: Value, log_settings: LogSettings) -> Assertion<Value> {
        // 1. retrieve the selector
        let mut selector = jsonpath::selector(&actual);
        // 2. run the json path query and encapsulate the result into a Value
        // TODO: handle the error as a test assertion failure.
        let result = json!(selector(&self.selector).unwrap());
        // 3. run the sub-expression
        self.expression
            .value
            .eval(result, self.expression.predicate.clone(), &log_settings)
            .assert(&log_settings)

        // call the corresponding predicate1
        // let assertion = Assertion {
        //     part: Part::JsonBody,
        //     predicate: self.expression.predicate,
        //     left: Hand::Left(self.expression.value),
        //     right: Hand::Right(actual),
        //     result: AssertionResult::NotYetStarted,
        // };

        // actual.json_path(selector, expression)
    }
}

// pub trait JsonBodyDsl<T> {

//     fn json_path(&self, )

//     fn eval(
//         &self,
//         actual: T,
//         predicate: Predicate,
//         log_settings: &LogSettings,
//     ) -> Assertion<Value> {
//         match predicate {
//             Predicate::Is => self.is(actual).assert(log_settings),
//             Predicate::IsNot => self.is_not(actual).assert(log_settings),
//             _ => unimplemented!("Invalid predicate for the json body DSL : {predicate}"),
//         }
//     }
// }
