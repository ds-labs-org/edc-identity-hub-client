use serde::{Deserialize, Serialize};

/// Mirrors `org.eclipse.edc.identityhub.spi.participantcontext.model.KeyPairUsage`.
/// Each variant's `#[serde(rename_all = "snake_case")]` spelling matches the
/// real enum's `@JsonProperty` values exactly (`sign_credentials`,
/// `sign_presentation`, `sign_token`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyPairUsage {
  SignCredentials,
  SignPresentation,
  SignToken,
}
