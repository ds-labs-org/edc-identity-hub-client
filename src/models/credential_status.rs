use serde::{Deserialize, Serialize};

/// Response body for `GET .../credentials/{credentialId}/status` — mirrors
/// `org.eclipse.edc.issuerservice.api.admin.credentials.v1.unstable.model.CredentialStatusResponse`.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialStatusResponse {
  pub credential_id: String,
  /// Free-text status, e.g. "active" / "suspended" / "revoked".
  pub status: String,
  pub reason: Option<String>,
}
