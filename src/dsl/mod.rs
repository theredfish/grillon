//! A domain-specific language organized into various modules providing built-in
//! types and functions for performing declarative assertions.

mod expression;
#[allow(clippy::wrong_self_convention)]
pub mod http;
mod part;

pub use self::expression::*;
pub use self::part::Part;
