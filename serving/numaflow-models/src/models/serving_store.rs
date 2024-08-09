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

/// ServingStore : ServingStore to track and store data and metadata for tracking and serving.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServingStore {
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<kube::core::Duration>,
    /// URL of the persistent store to write the callbacks
    #[serde(rename = "url")]
    pub url: String,
}

impl ServingStore {
    /// ServingStore to track and store data and metadata for tracking and serving.
    pub fn new(url: String) -> ServingStore {
        ServingStore { ttl: None, url }
    }
}
