#![deny(
    rust_2018_idioms,
    nonstandard_style,
    macro_use_extern_crate,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    trivial_casts,
    trivial_numeric_casts
)]
#![warn(missing_docs)]
#![forbid(non_ascii_idents, unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod assert;
mod assertion;
pub mod dsl;
mod error;
mod grillon;
pub mod request;
pub mod response;
mod url;

#[doc(inline)]
pub use self::{
    assert::Assert,
    error::{Error, Result},
    grillon::{Grillon, LogSettings},
    request::Request,
    response::Response,
};

pub use hyper::{header, Method, StatusCode};
pub use serde_json::{json, Value};
