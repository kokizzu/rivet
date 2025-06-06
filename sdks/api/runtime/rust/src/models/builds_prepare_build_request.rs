/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BuildsPrepareBuildRequest {
    /// A tag given to the project build.
    #[serde(rename = "image_tag", skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    #[serde(rename = "image_file")]
    pub image_file: Box<crate::models::UploadPrepareFile>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<crate::models::BuildsBuildKind>,
    #[serde(rename = "compression", skip_serializing_if = "Option::is_none")]
    pub compression: Option<crate::models::BuildsBuildCompression>,
}

impl BuildsPrepareBuildRequest {
    pub fn new(image_file: crate::models::UploadPrepareFile) -> BuildsPrepareBuildRequest {
        BuildsPrepareBuildRequest {
            image_tag: None,
            image_file: Box::new(image_file),
            kind: None,
            compression: None,
        }
    }
}


