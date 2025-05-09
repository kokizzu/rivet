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
pub struct CloudGamesNamespacesSetNamespaceCdnAuthTypeRequest {
	#[serde(rename = "auth_type")]
	pub auth_type: crate::models::CloudCdnAuthType,
}

impl CloudGamesNamespacesSetNamespaceCdnAuthTypeRequest {
	pub fn new(
		auth_type: crate::models::CloudCdnAuthType,
	) -> CloudGamesNamespacesSetNamespaceCdnAuthTypeRequest {
		CloudGamesNamespacesSetNamespaceCdnAuthTypeRequest { auth_type }
	}
}
