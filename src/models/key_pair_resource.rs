use crate::models::KeyPairUsage;
use serde::{Deserialize, Serialize};

/// Mirrors `org.eclipse.edc.identityhub.spi.keypair.model.KeyPairResource`
/// (fields inherited from `AbstractParticipantResource`/`Entity` -- `id`,
/// `createdAt`, `participantContextId` -- are flattened in here to match the
/// real JSON shape).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyPairResource {
  pub id: String,
  pub created_at: i64,
  pub participant_context_id: String,
  pub timestamp: i64,
  pub key_id: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub group_name: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub key_context: Option<String>,
  pub default_pair: bool,
  pub use_duration: i64,
  pub rotation_duration: i64,
  pub serialized_public_key: String,
  pub private_key_alias: String,
  /// `KeyPairState` code: `Created`=100, `Activated`=200, `Rotated`=300,
  /// `Revoked`=400.
  pub state: i32,
  #[serde(default)]
  pub usage: Vec<KeyPairUsage>,
}
