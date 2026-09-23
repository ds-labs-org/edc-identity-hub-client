use serde::{Deserialize, Serialize};

/// Request body for `POST .../credentials/offer` — mirrors
/// `org.eclipse.edc.issuerservice.api.admin.credentials.v1.unstable.model.CredentialOfferDto`.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialOfferDto {
  pub holder_id: String,
  pub credentials: Vec<String>,
}
