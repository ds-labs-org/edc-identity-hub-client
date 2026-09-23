use serde::{Deserialize, Serialize};

/// Response element for `POST .../credentials/query` — metadata about an
/// issued verifiable credential (no proof). Mirrors
/// `org.eclipse.edc.issuerservice.api.admin.credentials.v1.unstable.model.VerifiableCredentialResourceDto`.
///
/// `format` and `credential` are kept as opaque JSON/string rather than
/// modeled as the real `CredentialFormat`/`VerifiableCredential` Java types:
/// their exact shapes weren't available while writing this client. This
/// follows the same escape hatch already used elsewhere in this crate for
/// loosely-typed fields (e.g. `CredentialClaims::credential_subject`,
/// `ParticipantContext::additional_properties`).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifiableCredentialResourceDto {
  pub id: String,
  pub participant_context_id: String,
  pub format: String,
  pub credential: serde_json::Value,
}
