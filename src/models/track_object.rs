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
pub struct TrackObject {
    /// If you want to share a track, it should be local. If you want to play a track shared by a remote agent, it should be remote
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// mid associated to track's transceiver. It should be set with local tracks only
    #[serde(rename = "mid", skip_serializing_if = "Option::is_none")]
    pub mid: Option<String>,
    /// Session ID of the track owner. It should be set for remote tracks only
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Given name for the track
    #[serde(rename = "trackName", skip_serializing_if = "Option::is_none")]
    pub track_name: Option<String>,
}

impl TrackObject {
    pub fn new() -> TrackObject {
        TrackObject {
            location: None,
            mid: None,
            session_id: None,
            track_name: None,
        }
    }
}
/// If you want to share a track, it should be local. If you want to play a track shared by a remote agent, it should be remote
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Location {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "remote")]
    Remote,
}

impl Default for Location {
    fn default() -> Location {
        Self::Local
    }
}

