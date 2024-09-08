/*
 * Cloudflare Calls API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppsAppIdSessionsNewPost201Response {
    #[serde(rename = "sessionDescription", skip_serializing_if = "Option::is_none")]
    pub session_description: Option<Box<models::NewSessionResponseSessionDescription>>,
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl AppsAppIdSessionsNewPost201Response {
    pub fn new() -> AppsAppIdSessionsNewPost201Response {
        AppsAppIdSessionsNewPost201Response {
            session_description: None,
            session_id: None,
        }
    }
}

