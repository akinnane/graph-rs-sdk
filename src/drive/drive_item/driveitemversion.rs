use crate::drive::drive_item::identityset::IdentitySet;
use std::collections::BTreeMap;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/driveitemversion?view=odsp-graph-online
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct DriveItemVersionCollection {
    #[serde(rename = "@odata.context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    odata_context: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    versions: Option<Vec<DriveItemVersion>>,
}

impl Eq for DriveItemVersionCollection {}

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/driveitemversion?view=odsp-graph-online
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct DriveItemVersion {
    #[serde(rename = "@microsoft.graph.downloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    microsoft_graph_download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_by: Option<IdentitySet>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
}

impl Eq for DriveItemVersion {}
