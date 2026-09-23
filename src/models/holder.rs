use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A trust record for a DID the issuer is willing to receive credential
/// requests from (`org.eclipse.edc.issuerservice.spi.holder.model.Holder`
/// in the real v0.18.0 source). Note that EDC's `Holder` carries no
/// lifecycle/approval state of its own -- just identity, a free-form
/// `properties` map, an `anonymous` flag and a last-modified timestamp.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Holder {
  pub holder_id: String,
  #[serde(default)]
  pub participant_context_id: Option<String>,
  pub did: String,
  pub holder_name: String,
  #[serde(default)]
  pub anonymous: bool,
  #[serde(default)]
  pub properties: HashMap<String, serde_json::Value>,
  pub last_modified_at: i64,
}
