use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request body for creating or updating a holder
/// (`org.eclipse.edc.issuerservice.spi.holder.model.HolderDto`). The server
/// converts this into a `Holder` via `toHolder(participantContextId)`,
/// which is why it carries no `participantContextId`/`anonymous`/
/// `lastModifiedAt` of its own -- those are resolved server-side.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HolderDto {
  pub id: String,
  pub did: String,
  pub name: String,
  #[serde(default, skip_serializing_if = "HashMap::is_empty")]
  pub properties: HashMap<String, serde_json::Value>,
}

impl HolderDto {
  pub fn new(id: String, did: String, name: String) -> Self {
    Self {
      id,
      did,
      name,
      properties: HashMap::new(),
    }
  }

  pub fn with_properties(mut self, properties: HashMap<String, serde_json::Value>) -> Self {
    self.properties = properties;
    self
  }
}
