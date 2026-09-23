//! Client for the Issuer Service's issuer-admin-api: verifiable-credential
//! lifecycle (query / status / revoke / suspend / resume / offer) and
//! issuance-process monitoring (query / get-by-id). Every resource here is
//! participant-scoped and version-segmented exactly like `IdentityHubClient`,
//! so this is a dedicated struct with the same `endpoint` + `version` +
//! `bearer_token` shape rather than an extension of `IssuerServiceClient`
//! (which fronts only the unauthenticated, unversioned `/metadata` endpoint
//! and whose `get_metadata()` signature other consumers may already rely on).

use crate::models::{
  CredentialOfferDto, CredentialStatusResponse, IssuanceProcessDto, QuerySpec,
  VerifiableCredentialResourceDto,
};
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

  pub async fn revoke_credential(
    &self,
    participant_context_id: &str,
    credential_id: &str,
  ) -> Result<()> {
    let url = format!(
      "{}/api/issuer/{}/participants/{participant_context_id}/credentials/{credential_id}/revoke",
      self.endpoint, self.version
    );
    let request_builder = self.client.post(&url);

    let request_builder = if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    };

    let response = request_builder.send().await?;

    if response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  /// Suspends a credential. Routed but stubbed as `501 Not Implemented` in
  /// the real v0.18.0 IdentityHub server as of this writing - callers get
  /// back `Err(IdentityHubClientError::Response(_))` until that lands.
  pub async fn suspend_credential(
    &self,
    participant_context_id: &str,
    credential_id: &str,
  ) -> Result<()> {
    let url = format!(
      "{}/api/issuer/{}/participants/{participant_context_id}/credentials/{credential_id}/suspend",
      self.endpoint, self.version
    );
    let request_builder = self.client.post(&url);

    let request_builder = if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    };

    let response = request_builder.send().await?;

    if response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  /// Resumes a suspended credential. Same 501-stub caveat as
  /// `suspend_credential` above.
  pub async fn resume_credential(
    &self,
    participant_context_id: &str,
    credential_id: &str,
  ) -> Result<()> {
    let url = format!(
      "{}/api/issuer/{}/participants/{participant_context_id}/credentials/{credential_id}/resume",
      self.endpoint, self.version
    );
    let request_builder = self.client.post(&url);

    let request_builder = if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    };

    let response = request_builder.send().await?;

    if response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn create_credential_offer(
    &self,
    participant_context_id: &str,
    offer: &CredentialOfferDto,
  ) -> Result<()> {
    let url = format!(
      "{}/api/issuer/{}/participants/{participant_context_id}/credentials/offer",
      self.endpoint, self.version
    );
    let request_builder = self.client.post(&url);

    let request_builder = if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    };

    let response = request_builder.json(offer).send().await?;

    if response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn query_issuance_processes(
    &self,
    participant_context_id: &str,
    query: &QuerySpec,
  ) -> Result<Vec<IssuanceProcessDto>> {
    let url = format!(
      "{}/api/issuer/{}/participants/{participant_context_id}/issuanceprocesses/query",
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

  pub async fn get_issuance_process(
    &self,
    participant_context_id: &str,
    issuance_process_id: &str,
  ) -> Result<IssuanceProcessDto> {
    let url = format!(
      "{}/api/issuer/{}/participants/{participant_context_id}/issuanceprocesses/{issuance_process_id}",
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
  use crate::models::{CredentialOfferDto, QuerySpec};
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

  // NOTE: in the real v0.18.0 server, POST .../suspend always answers 501
  // Not Implemented (routed but stubbed) - see IssuerCredentialsAdminApiController
  // in the EDC IdentityHub source. This test mocks a 200 to pin down the
  // request shape suspend_credential() sends once the server implements it;
  // against today's real deployment the call will surface as
  // Err(IdentityHubClientError::Response(_)) with status 501, which callers
  // must handle explicitly (e.g. by disabling the Suspend action in the UI).
  #[tokio::test]
  async fn suspend_credential_posts_to_suspend() {
    let mock_server = MockServer::start().await;
    let participant_context_id = "participant-1";
    let credential_id = "cred-1";

    Mock::given(method("POST"))
      .and(path(format!(
        "/api/issuer/v1beta/participants/{participant_context_id}/credentials/{credential_id}/suspend"
      )))
      .respond_with(ResponseTemplate::new(200))
      .mount(&mock_server)
      .await;

    let client = super::IssuerAdminApiClient::new(
      reqwest::Client::new(),
      mock_server.uri(),
      None,
      IdentityHubClientVersion::V1Beta,
    );

    client
      .suspend_credential(participant_context_id, credential_id)
      .await
      .expect("suspend_credential should succeed against a server that implements it");
  }

  // Same 501-stub caveat as suspend_credential above.
  #[tokio::test]
  async fn resume_credential_posts_to_resume() {
    let mock_server = MockServer::start().await;
    let participant_context_id = "participant-1";
    let credential_id = "cred-1";

    Mock::given(method("POST"))
      .and(path(format!(
        "/api/issuer/v1beta/participants/{participant_context_id}/credentials/{credential_id}/resume"
      )))
      .respond_with(ResponseTemplate::new(200))
      .mount(&mock_server)
      .await;

    let client = super::IssuerAdminApiClient::new(
      reqwest::Client::new(),
      mock_server.uri(),
      None,
      IdentityHubClientVersion::V1Beta,
    );

    client
      .resume_credential(participant_context_id, credential_id)
      .await
      .expect("resume_credential should succeed against a server that implements it");
  }

  #[tokio::test]
  async fn create_credential_offer_posts_the_offer_body() {
    let mock_server = MockServer::start().await;
    let participant_context_id = "participant-1";
    let offer = CredentialOfferDto {
      holder_id: "holder-1".to_string(),
      credentials: vec!["credential-definition-1".to_string()],
    };

    Mock::given(method("POST"))
      .and(path(format!(
        "/api/issuer/v1beta/participants/{participant_context_id}/credentials/offer"
      )))
      .and(body_json(&offer))
      .respond_with(ResponseTemplate::new(200))
      .mount(&mock_server)
      .await;

    let client = super::IssuerAdminApiClient::new(
      reqwest::Client::new(),
      mock_server.uri(),
      None,
      IdentityHubClientVersion::V1Beta,
    );

    client
      .create_credential_offer(participant_context_id, &offer)
      .await
      .expect("create_credential_offer should succeed");
  }

  #[tokio::test]
  async fn query_issuance_processes_posts_query_spec_and_returns_matching_processes() {
    let mock_server = MockServer::start().await;
    let participant_context_id = "participant-1";
    let query = QuerySpec::none();

    let response_body = serde_json::json!([
      {
        "id": "issuance-1",
        "holderId": "holder-1",
        "participantContextId": participant_context_id,
        "holderPid": "holder-pid-1",
        "claims": {"name": "Alice"},
        "credentialDefinitions": ["credential-definition-1"],
        "credentialFormats": {"credential-definition-1": "VC1_0_JWT"},
        "state": "DELIVERED",
        "createdAt": 1_700_000_000_000i64,
        "updatedAt": 1_700_000_001_000i64
      }
    ]);

    Mock::given(method("POST"))
      .and(path(format!(
        "/api/issuer/v1beta/participants/{participant_context_id}/issuanceprocesses/query"
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

    let processes = client
      .query_issuance_processes(participant_context_id, &query)
      .await
      .expect("query_issuance_processes should succeed");

    assert_eq!(processes.len(), 1);
    assert_eq!(processes[0].id, "issuance-1");
    assert_eq!(processes[0].state, "DELIVERED");
  }

  #[tokio::test]
  async fn get_issuance_process_gets_by_id() {
    let mock_server = MockServer::start().await;
    let participant_context_id = "participant-1";
    let issuance_process_id = "issuance-1";

    Mock::given(method("GET"))
      .and(path(format!(
        "/api/issuer/v1beta/participants/{participant_context_id}/issuanceprocesses/{issuance_process_id}"
      )))
      .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "id": issuance_process_id,
        "holderId": "holder-1",
        "participantContextId": participant_context_id,
        "holderPid": "holder-pid-1",
        "claims": {},
        "credentialDefinitions": [],
        "credentialFormats": {},
        "state": "APPROVED",
        "createdAt": 1_700_000_000_000i64,
        "updatedAt": 1_700_000_000_000i64
      })))
      .mount(&mock_server)
      .await;

    let client = super::IssuerAdminApiClient::new(
      reqwest::Client::new(),
      mock_server.uri(),
      None,
      IdentityHubClientVersion::V1Beta,
    );

    let process = client
      .get_issuance_process(participant_context_id, issuance_process_id)
      .await
      .expect("get_issuance_process should succeed");

    assert_eq!(process.id, issuance_process_id);
    assert_eq!(process.state, "APPROVED");
  }
}
