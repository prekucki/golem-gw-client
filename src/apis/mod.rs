use failure::Fail;
use hyper;
use serde;
use serde_json;

#[derive(Debug, Fail)]
pub enum Error<T: std::fmt::Debug + Send + Sync + 'static> {
    #[fail(display = "invalid uri {}", _0)]
    UriError(hyper::http::uri::InvalidUri),
    #[fail(display = "{}", _0)]
    Hyper(hyper::Error),
    #[fail(display = "{}", _0)]
    ToStrError(hyper::header::ToStrError),
    #[fail(display = "invalid header name, {}", _0)]
    InvalidHeaderName(hyper::header::InvalidHeaderName),
    #[fail(display = "invalid header value, {}", _0)]
    InvalidHeaderValue(hyper::header::InvalidHeaderValue),
    #[fail(display = "serde: {}", _0)]
    Serde(serde_json::Error),
    #[fail(display = "api error: {:?}", _0)]
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T: std::fmt::Debug + Send + Sync> From<(hyper::StatusCode, &'de [u8])> for Error<T>
where
    T: serde::Deserialize<'de>,
{
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError {
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError {
                code: e.0,
                content: Some(t),
            }),
            Err(e) => Error::from(e),
        }
    }
}

impl<T: std::fmt::Debug + Send + Sync> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e);
    }
}

impl<T: std::fmt::Debug + Send + Sync> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e);
    }
}

use super::models::*;

mod request;

mod default_api;
pub use self::default_api::{DefaultApi, DefaultApiClient};
use std::fmt::Debug;

pub mod client;
pub mod configuration;
