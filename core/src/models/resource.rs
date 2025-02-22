use serde_derive::{Deserialize, Serialize};
use crate::models::authentication::*;

/// Represents the definition of an external resource
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalResourceDefinition{

    /// Gets/sets the external resource's name, if any
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Gets/sets the endpoint at which to get the defined resource
    #[serde(rename = "endpoint")]
    pub endpoint: OneOfEndpointDefinitionOrUri

}

/// Represents the definition of an endpoint
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDefinition{

    /// Gets/sets the endpoint's uri
    #[serde(rename = "uri")]
    pub uri : String,

    /// Gets/sets the endpoint's authentication policy, if any
    #[serde(rename = "authentication", skip_serializing_if = "Option::is_none")]
    pub authentication: Option<AuthenticationPolicyDefinition>

}

/// Represents a value that can be either an EndpointDefinition or an Uri
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOfEndpointDefinitionOrUri{
    /// Variant holding an EndpointDefinition
    Endpoint(EndpointDefinition),
    /// Variant holding a URL
    Uri(String),
}
impl Default for OneOfEndpointDefinitionOrUri {
    fn default() -> Self {
        // Choose a default variant. For example, default to an empty Uri.
        OneOfEndpointDefinitionOrUri::Uri(String::new())
    }
}