use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response element for the issuer-admin-api issuance-process endpoints
/// (`GET .../issuanceprocesses/{id}` and `POST .../issuanceprocesses/query`).
/// Mirrors
/// `org.eclipse.edc.issuerservice.api.admin.issuance.v1.unstable.model.IssuanceProcessDto`.
///
/// `credential_formats` values and `claims` are kept as opaque
/// string/JSON — see `VerifiableCredentialResourceDto` for why.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssuanceProcessDto {
  pub id: String,
  pub holder_id: String,
  pub participant_context_id: String,
  pub holder_pid: String,
  pub claims: HashMap<String, serde_json::Value>,
  pub credential_definitions: Vec<String>,
  pub credential_formats: HashMap<String, String>,
  pub state: String,
  #[serde(with = "ts_milliseconds")]
  pub created_at: DateTime<Utc>,
  #[serde(with = "ts_milliseconds")]
  pub updated_at: DateTime<Utc>,
}
