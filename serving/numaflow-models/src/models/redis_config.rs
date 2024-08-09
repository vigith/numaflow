/*
Copyright 2022 The Numaproj Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

// Code generated by Openapi Generator. DO NOT EDIT.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisConfig {
    /// Only required when Sentinel is used
    #[serde(rename = "masterName", skip_serializing_if = "Option::is_none")]
    pub master_name: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<k8s_openapi::api::core::v1::SecretKeySelector>,
    #[serde(rename = "sentinelPassword", skip_serializing_if = "Option::is_none")]
    pub sentinel_password: Option<k8s_openapi::api::core::v1::SecretKeySelector>,
    /// Sentinel URL, will be ignored if Redis URL is provided
    #[serde(rename = "sentinelUrl", skip_serializing_if = "Option::is_none")]
    pub sentinel_url: Option<String>,
    /// Redis URL
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Redis user
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl RedisConfig {
    pub fn new() -> RedisConfig {
        RedisConfig {
            master_name: None,
            password: None,
            sentinel_password: None,
            sentinel_url: None,
            url: None,
            user: None,
        }
    }
}
