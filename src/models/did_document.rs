use serde::{Deserialize, Serialize};

/// Mirrors `org.eclipse.edc.iam.did.spi.document.DidDocument`, as returned in
/// the `Collection<DidDocument>` body of `POST .../dids/query`.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidDocument {
  pub id: String,
  #[serde(rename = "@context", default)]
  pub context: Vec<serde_json::Value>,
  #[serde(default)]
  pub service: Vec<DidDocumentService>,
  #[serde(default)]
  pub verification_method: Vec<VerificationMethod>,
  #[serde(default)]
  pub authentication: Vec<String>,
  #[serde(default)]
  pub capability_invocation: Vec<String>,
}

/// Mirrors `org.eclipse.edc.iam.did.spi.document.Service` (a DID document's
/// `service` entry, e.g. a DCP `CredentialService` endpoint).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidDocumentService {
  pub id: String,
  #[serde(rename = "type")]
  pub service_type: String,
  pub service_endpoint: String,
}

/// Mirrors `org.eclipse.edc.iam.did.spi.document.VerificationMethod`. Exactly
/// one of `public_key_multibase` / `public_key_jwk` is present on any real
/// document.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationMethod {
  pub id: String,
  #[serde(rename = "type")]
  pub method_type: String,
  pub controller: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub public_key_multibase: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub public_key_jwk: Option<serde_json::Value>,
}
