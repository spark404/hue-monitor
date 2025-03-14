use super::Request;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Config {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub name: String,
    pub datastoreversion: String,
    pub swversion: String,
    pub apiversion: String,
    pub mac: String,
    pub bridgeid: String,
    pub factorynew: bool,
    pub replacesbridgeid: Option<String>,
    pub modelid: String,
    pub starterkitid: Option<String>,
}

impl Request for Config {
    type Response = ConfigResponse;
    type Body = ();
    fn endpoint(&self) -> Cow<str> {
        Cow::from("/api/config")
    }
}