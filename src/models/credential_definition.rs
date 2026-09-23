use serde::{Deserialize, Serialize};

/// Domain object returned by the issuer-admin-api's credential-definition endpoints
/// (`CredentialDefinition` in the real EDC v0.18.0 source, extending
/// `AbstractParticipantResource` for `id`/`participantContextId`).
///
/// See `CredentialDefinitionDto` for the note on `rules`/`mappings` being modeled as
/// opaque JSON.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialDefinition {
  pub id: String,
  pub participant_context_id: String,
  pub credential_type: String,
  pub format: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub json_schema: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub json_schema_url: Option<String>,
  pub validity: i64,
  #[serde(default)]
  pub attestations: Vec<String>,
  #[serde(default)]
  pub additional_context: Vec<String>,
  #[serde(default)]
  pub rules: Vec<serde_json::Value>,
  #[serde(default)]
  pub mappings: Vec<serde_json::Value>,
}
