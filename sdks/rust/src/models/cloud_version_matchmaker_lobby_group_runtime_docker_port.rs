/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerLobbyGroupRuntimeDockerPort : **Deprecated: use GameMode instead** A docker port.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudVersionMatchmakerLobbyGroupRuntimeDockerPort {
    /// The label of this docker port.
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "port_range", skip_serializing_if = "Option::is_none")]
    pub port_range: Option<Box<crate::models::CloudVersionMatchmakerPortRange>>,
    #[serde(rename = "proxy_protocol")]
    pub proxy_protocol: crate::models::CloudVersionMatchmakerPortProtocol,
    /// The port number to connect to.
    #[serde(rename = "target_port", skip_serializing_if = "Option::is_none")]
    pub target_port: Option<i32>,
}

impl CloudVersionMatchmakerLobbyGroupRuntimeDockerPort {
    /// **Deprecated: use GameMode instead** A docker port.
    pub fn new(label: String, proxy_protocol: crate::models::CloudVersionMatchmakerPortProtocol) -> CloudVersionMatchmakerLobbyGroupRuntimeDockerPort {
        CloudVersionMatchmakerLobbyGroupRuntimeDockerPort {
            label,
            port_range: None,
            proxy_protocol,
            target_port: None,
        }
    }
}

