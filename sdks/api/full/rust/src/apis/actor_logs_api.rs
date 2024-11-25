/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`actor_logs_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActorLogsGetError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// Returns the logs for a given actor.
pub async fn actor_logs_get(
	configuration: &configuration::Configuration,
	actor: &str,
	stream: crate::models::ActorLogStream,
	project: Option<&str>,
	environment: Option<&str>,
	watch_index: Option<&str>,
) -> Result<crate::models::ActorGetActorLogsResponse, Error<ActorLogsGetError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/actors/{actor}/logs",
		local_var_configuration.base_path,
		actor = crate::apis::urlencode(actor)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

	if let Some(ref local_var_str) = project {
		local_var_req_builder =
			local_var_req_builder.query(&[("project", &local_var_str.to_string())]);
	}
	if let Some(ref local_var_str) = environment {
		local_var_req_builder =
			local_var_req_builder.query(&[("environment", &local_var_str.to_string())]);
	}
	local_var_req_builder = local_var_req_builder.query(&[("stream", &stream.to_string())]);
	if let Some(ref local_var_str) = watch_index {
		local_var_req_builder =
			local_var_req_builder.query(&[("watch_index", &local_var_str.to_string())]);
	}
	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<ActorLogsGetError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}