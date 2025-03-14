use crate::hue::requests::{GenericResponse, Request};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DevicesRequest {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceRequest {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProductData {
    model_id: String,
    manufacturer_name: String,
    product_name: String,
    product_archetype: String,
    certified: bool,
    software_version: String,
    hardware_platform_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MetaData {
    pub name: String,
    pub archetype: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Usertest {
    status: String,
    usertest: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceModeType {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceMode {
    status: String,
    mode: DeviceModeType,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identify {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceIdentifier {
    pub rid: String,
    pub rtype: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "type")]
    pub device_type: String,
    pub id: String,
    pub id_v1: Option<String>,
    pub product_data: ProductData,
    pub metadata: MetaData,
    pub identify: Option<Identify>,
    pub usertest: Option<Usertest>,
    pub device_mode: Option<DeviceMode>,
    pub services: Vec<ResourceIdentifier>,
}

impl Request for DevicesRequest {
    type Response = GenericResponse<Device>;
    type Body = ();

    fn endpoint(&self) -> Cow<str> {
        "/clip/v2/resource/device".into()
    }
}

impl Request for DeviceRequest {
    type Response = GenericResponse<Device>;
    type Body = ();

    fn endpoint(&self) -> Cow<str> {
        format!("/clip/v2/resource/device/{}", self.id).into()
    }
}
