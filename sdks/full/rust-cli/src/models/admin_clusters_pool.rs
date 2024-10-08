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
pub struct AdminClustersPool {
    #[serde(rename = "desired_count")]
    pub desired_count: i32,
    #[serde(rename = "drain_timeout_ms")]
    pub drain_timeout_ms: i64,
    #[serde(rename = "hardware")]
    pub hardware: Vec<crate::models::AdminClustersHardware>,
    #[serde(rename = "max_count")]
    pub max_count: i32,
    #[serde(rename = "min_count")]
    pub min_count: i32,
    #[serde(rename = "pool_type")]
    pub pool_type: crate::models::AdminClustersPoolType,
}

impl AdminClustersPool {
    pub fn new(desired_count: i32, drain_timeout_ms: i64, hardware: Vec<crate::models::AdminClustersHardware>, max_count: i32, min_count: i32, pool_type: crate::models::AdminClustersPoolType) -> AdminClustersPool {
        AdminClustersPool {
            desired_count,
            drain_timeout_ms,
            hardware,
            max_count,
            min_count,
            pool_type,
        }
    }
}


