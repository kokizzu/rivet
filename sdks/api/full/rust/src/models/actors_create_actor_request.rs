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
pub struct ActorsCreateActorRequest {
	#[serde(rename = "region", skip_serializing_if = "Option::is_none")]
	pub region: Option<String>,
	#[serde(rename = "tags", deserialize_with = "Option::deserialize")]
	pub tags: Option<serde_json::Value>,
	#[serde(rename = "build", skip_serializing_if = "Option::is_none")]
	pub build: Option<uuid::Uuid>,
	#[serde(
		rename = "build_tags",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub build_tags: Option<Option<serde_json::Value>>,
	#[serde(rename = "runtime", skip_serializing_if = "Option::is_none")]
	pub runtime: Option<Box<crate::models::ActorsCreateActorRuntimeRequest>>,
	#[serde(rename = "network", skip_serializing_if = "Option::is_none")]
	pub network: Option<Box<crate::models::ActorsCreateActorNetworkRequest>>,
	#[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
	pub resources: Option<Box<crate::models::ActorsResources>>,
	#[serde(rename = "lifecycle", skip_serializing_if = "Option::is_none")]
	pub lifecycle: Option<Box<crate::models::ActorsLifecycle>>,
}

impl ActorsCreateActorRequest {
	pub fn new(tags: Option<serde_json::Value>) -> ActorsCreateActorRequest {
		ActorsCreateActorRequest {
			region: None,
			tags,
			build: None,
			build_tags: None,
			runtime: None,
			network: None,
			resources: None,
			lifecycle: None,
		}
	}
}
