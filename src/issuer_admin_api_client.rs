use crate::errors::{IdentityHubClientError, Result};
use crate::models::CredentialDefinitionDto;
use crate::IdentityHubClientVersion;

/// Client for the issuer-admin-api's participant-scoped, version-segmented resources
/// (credential definitions today; holders, credentials, and issuance processes are
/// expected to land here from sibling work). Mirrors `IdentityHubClient`'s shape
/// (endpoint + version + optional bearer token) rather than extending
/// `IssuerServiceClient`, whose `get_metadata` hits an unversioned, unauthenticated
/// discovery endpoint and is intentionally left untouched.
pub struct IssuerAdminApiClient {
  client: reqwest::Client,
  endpoint: String,
  bearer_token: Option<String>,
  version: IdentityHubClientVersion,
}

impl IssuerAdminApiClient {
  pub fn new(
    client: reqwest::Client,
    endpoint: String,
    bearer_token: Option<String>,
    version: IdentityHubClientVersion,
  ) -> Self {
    Self {
      client,
      endpoint,
      bearer_token,
      version,
    }
  }

  pub async fn create_credential_definition(
    &self,
    participant_context_id: &str,
    credential_definition: &CredentialDefinitionDto,
  ) -> Result<()> {
    let url = format!(
      "{}/api/issuer/{}/participants/{participant_context_id}/credentialdefinitions",
      self.endpoint, self.version
    );
    let request_builder = self.client.post(&url);

    let request_builder = if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    };

    let response = request_builder.json(credential_definition).send().await?;

    if response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use wiremock::matchers::{body_json, header, method, path};
  use wiremock::{Mock, MockServer, ResponseTemplate};

  fn dto() -> CredentialDefinitionDto {
    CredentialDefinitionDto::new(
      Some("cred-def-1".to_string()),
      "MembershipCredential".to_string(),
      Some("vc1_0_jwt".to_string()),
      Some("{\"type\":\"object\"}".to_string()),
      None,
      3600,
      vec!["attestation-1".to_string()],
      vec![],
      vec![],
    )
  }

  #[tokio::test]
  async fn create_credential_definition_posts_to_credentialdefinitions() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
      .and(path(
        "/api/issuer/v1beta/participants/participant-1/credentialdefinitions",
      ))
      .and(header("Authorization", "Bearer test-token"))
      .and(body_json(&dto()))
      .respond_with(ResponseTemplate::new(201))
      .mount(&server)
      .await;

    let client = IssuerAdminApiClient::new(
      reqwest::Client::new(),
      server.uri(),
      Some("test-token".to_string()),
      IdentityHubClientVersion::V1Beta,
    );

    client
      .create_credential_definition("participant-1", &dto())
      .await
      .expect("create_credential_definition should succeed");
  }
}
