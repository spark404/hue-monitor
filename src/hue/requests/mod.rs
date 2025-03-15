mod config;
mod devices;
mod temperature;
mod light_level;

use crate::hue::error::Error;
pub use config::*;
pub use devices::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
pub use temperature::*;
pub use light_level::*;

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

pub trait Id {
    fn id(&self) -> Cow<str>;
}

pub trait GetById<T> {
    fn get_by_id(&self, id: &String) -> Option<&T>;
}

impl<T: Id> GetById<T> for GenericResponse<T> {
    fn get_by_id(&self, id: &String) -> Option<&T> {
        self.data.iter().find(|x| x.id().to_string().eq(id))
    }
}