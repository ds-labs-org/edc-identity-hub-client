use crate::models::KeyPairUsage;
use serde::{Deserialize, Serialize};

/// Mirrors `org.eclipse.edc.identityhub.spi.participantcontext.model.KeyDescriptor`
/// -- the request body for `PUT .../keypairs` and the optional body of
/// `.../rotate` and `.../revoke`. Exactly one of `public_key_jwk` /
/// `public_key_pem` / `key_generator_params` should be set; the server
/// rejects specifying more than one (or none).
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyDescriptor {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub resource_id: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub key_id: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub r#type: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub private_key_alias: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub public_key_jwk: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub public_key_pem: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub key_generator_params: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub is_active: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub usage: Option<Vec<KeyPairUsage>>,
}
