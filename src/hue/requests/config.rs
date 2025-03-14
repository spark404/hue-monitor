use super::Request;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Config {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigResponse {
    name: String,
    datastoreversion: String,
    swversion: String,
    apiversion: String,
    mac: String,
    bridgeid: String,
    factorynew: bool,
    replacesbridgeid: Option<String>,
    modelid: String,
    starterkitid: Option<String>,
}

impl Request for Config {
    type Response = ConfigResponse;
    type Body = ();
    fn endpoint(&self) -> Cow<str> {
        Cow::from("/api/config")
    }
}