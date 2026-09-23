//! Client for the Issuer Service's issuer-admin-api: verifiable-credential
//! lifecycle (query / status / revoke / suspend / resume / offer) and
//! issuance-process monitoring (query / get-by-id). Every resource here is
//! participant-scoped and version-segmented exactly like `IdentityHubClient`,
//! so this is a dedicated struct with the same `endpoint` + `version` +
//! `bearer_token` shape rather than an extension of `IssuerServiceClient`
//! (which fronts only the unauthenticated, unversioned `/metadata` endpoint
//! and whose `get_metadata()` signature other consumers may already rely on).

use crate::models::{CredentialStatusResponse, QuerySpec, VerifiableCredentialResourceDto};
use crate::{IdentityHubClientError, IdentityHubClientVersion, Result};

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

  pub async fn query_credentials(
    &self,
    participant_context_id: &str,
    query: &QuerySpec,
  ) -> Result<Vec<VerifiableCredentialResourceDto>> {
    let url = format!(
      "{}/api/issuer/{}/participants/{participant_context_id}/credentials/query",
      self.endpoint, self.version
    );
    let request_builder = self.client.post(&url);

    let request_builder = if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    };

    let response = request_builder.json(query).send().await?;

    if response.status().is_success() {
      Ok(response.json().await?)
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn get_credential_status(
    &self,
    participant_context_id: &str,
    credential_id: &str,
  ) -> Result<CredentialStatusResponse> {
    let url = format!(
      "{}/api/issuer/{}/participants/{participant_context_id}/credentials/{credential_id}/status",
      self.endpoint, self.version
    );
    let request_builder = self.client.get(&url);

    let request_builder = if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    };

    let response = request_builder.send().await?;

    if response.status().is_success() {
      Ok(response.json().await?)
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::IdentityHubClientVersion;
  use crate::models::QuerySpec;
  use wiremock::matchers::{body_json, header, method, path};
  use wiremock::{Mock, MockServer, ResponseTemplate};

  #[tokio::test]
  async fn query_credentials_posts_query_spec_and_returns_matching_resources() {
    let mock_server = MockServer::start().await;
    let participant_context_id = "participant-1";
    let query = QuerySpec::none();

    let response_body = serde_json::json!([
      {
        "id": "cred-1",
        "participantContextId": participant_context_id,
        "format": "VC1_0_JWT",
        "credential": { "type": ["VerifiableCredential"] }
      }
    ]);

    Mock::given(method("POST"))
      .and(path(format!(
        "/api/issuer/v1beta/participants/{participant_context_id}/credentials/query"
      )))
      .and(body_json(&query))
      .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
      .mount(&mock_server)
      .await;

    let client = super::IssuerAdminApiClient::new(
      reqwest::Client::new(),
      mock_server.uri(),
      None,
      IdentityHubClientVersion::V1Beta,
    );

    let credentials = client
      .query_credentials(participant_context_id, &query)
      .await
      .expect("query_credentials should succeed");

    assert_eq!(credentials.len(), 1);
    assert_eq!(credentials[0].id, "cred-1");
    assert_eq!(credentials[0].format, "VC1_0_JWT");
  }

  #[tokio::test]
  async fn get_credential_status_gets_the_status_endpoint() {
    let mock_server = MockServer::start().await;
    let participant_context_id = "participant-1";
    let credential_id = "cred-1";

    Mock::given(method("GET"))
      .and(path(format!(
        "/api/issuer/v1beta/participants/{participant_context_id}/credentials/{credential_id}/status"
      )))
      .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "credentialId": credential_id,
        "status": "active",
        "reason": null
      })))
      .mount(&mock_server)
      .await;

    let client = super::IssuerAdminApiClient::new(
      reqwest::Client::new(),
      mock_server.uri(),
      None,
      IdentityHubClientVersion::V1Beta,
    );

    let status = client
      .get_credential_status(participant_context_id, credential_id)
      .await
      .expect("get_credential_status should succeed");

    assert_eq!(status.credential_id, credential_id);
    assert_eq!(status.status, "active");
    assert_eq!(status.reason, None);
  }

  #[tokio::test]
  async fn revoke_credential_posts_to_revoke_with_bearer_token() {
    let mock_server = MockServer::start().await;
    let participant_context_id = "participant-1";
    let credential_id = "cred-1";

    Mock::given(method("POST"))
      .and(path(format!(
        "/api/issuer/v1beta/participants/{participant_context_id}/credentials/{credential_id}/revoke"
      )))
      .and(header("Authorization", "Bearer test-token"))
      .respond_with(ResponseTemplate::new(200))
      .mount(&mock_server)
      .await;

    let client = super::IssuerAdminApiClient::new(
      reqwest::Client::new(),
      mock_server.uri(),
      Some("test-token".to_string()),
      IdentityHubClientVersion::V1Beta,
    );

    client
      .revoke_credential(participant_context_id, credential_id)
      .await
      .expect("revoke_credential should succeed");
  }
}
