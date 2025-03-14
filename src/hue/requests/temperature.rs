use crate::hue::requests::{GenericResponse, Request, ResourceIdentifier};
use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TemperaturesRequest {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TemperatureRequest {
    pub(crate) id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TemperatureReport {
    changed: String,
    temperature: Number,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TemperatureData {
    pub temperature: Number,
    pub temperature_valid: bool,
    pub temperature_report: TemperatureReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Temperature {
    #[serde(rename = "type")]
    pub device_type: String,
    pub id: String,
    pub id_v1: Option<String>,
    pub owner: ResourceIdentifier,
    pub enabled: bool,
    pub temperature: TemperatureData,
}

impl Request for TemperaturesRequest {
    type Response = GenericResponse<Temperature>;
    type Body = ();

    fn endpoint(&self) -> Cow<str> {
        "/clip/v2/resource/temperature".into()
    }
}

impl Request for TemperatureRequest {
    type Response = GenericResponse<Temperature>;
    type Body = ();

    fn endpoint(&self) -> Cow<str> {
        format!("/clip/v2/resource/temperature/{}", self.id).into()
    }
}