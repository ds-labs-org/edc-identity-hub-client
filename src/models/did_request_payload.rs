use serde::{Deserialize, Serialize};

/// The request body every `identity-api` DID-management endpoint that takes
/// a bare DID accepts (`publish`, `unpublish`, `state`) -- mirrors the
/// real controller's `DidRequestPayload` record (`{"did": "<did string>"}`).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidRequestPayload {
  pub did: String,
}

impl DidRequestPayload {
  pub fn new(did: impl Into<String>) -> Self {
    Self { did: did.into() }
  }
}
