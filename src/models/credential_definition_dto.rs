use serde::{Deserialize, Serialize};

/// Request payload for creating or updating a credential definition via the
/// issuer-admin-api (`CredentialDefinitionDto` in the real EDC v0.18.0 source).
///
/// `rules` and `mappings` are modeled as opaque JSON values here: the real
/// `CredentialRuleDefinition` and `MappingDefinition` types are referenced by the
/// controller but weren't part of the source lookup that backed this client, so
/// round-tripping arbitrary JSON avoids asserting a shape that hasn't been verified.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialDefinitionDto {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  pub credential_type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub format: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub json_schema: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub json_schema_url: Option<String>,
  pub validity: i64,
  #[serde(default)]
  pub attestations: Vec<String>,
  #[serde(default)]
  pub rules: Vec<serde_json::Value>,
  #[serde(default)]
  pub mappings: Vec<serde_json::Value>,
}

impl CredentialDefinitionDto {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    id: Option<String>,
    credential_type: String,
    format: Option<String>,
    json_schema: Option<String>,
    json_schema_url: Option<String>,
    validity: i64,
    attestations: Vec<String>,
    rules: Vec<serde_json::Value>,
    mappings: Vec<serde_json::Value>,
  ) -> Self {
    Self {
      id,
      credential_type,
      format,
      json_schema,
      json_schema_url,
      validity,
      attestations,
      rules,
      mappings,
    }
  }
}
