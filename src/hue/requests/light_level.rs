use crate::hue::requests::{GenericResponse, Request, ResourceIdentifier};
use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LightLevelRequest {
    pub(crate) id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LightLevelReport {
    changed: String,
    light_level: Number,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LightLevelData {
    pub light_level: Number,
    pub light_level_valid: bool,
    pub light_level_report: LightLevelReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LightLevel {
    #[serde(rename = "type")]
    pub device_type: String,
    pub id: String,
    pub id_v1: Option<String>,
    pub owner: ResourceIdentifier,
    pub enabled: bool,
    pub light: LightLevelData,
}

impl Request for LightLevelRequest {
    type Response = GenericResponse<LightLevel>;
    type Body = ();

    fn endpoint(&self) -> Cow<str> {
        match &self.id {
            None => "/clip/v2/resource/light_level".into(),
            Some(entity_id) => format!("/clip/v2/resource/light_level/{}", entity_id).into(),
        }
    }
}

impl Default for LightLevelRequest {
    fn default() -> Self {
        LightLevelRequest { id: None }
    }
}