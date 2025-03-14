mod config;
mod devices;
mod temperature;

use crate::hue::error::Error;
pub use config::*;
pub use devices::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
pub use temperature::*;

pub trait Request {
    type Response: for<'de> Deserialize<'de>;
    const METHOD: Method = Method::GET;

    type Body: Serialize;

    fn endpoint(&self) -> Cow<str>;
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericResponse<T> {
    pub errors: Vec<String>,
    pub data: Vec<T>,
}