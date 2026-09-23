//! Client for the Issuer Service's issuer-admin-api: credential definitions,
//! holders, verifiable-credential lifecycle (query / status / revoke /
//! suspend / resume / offer), and issuance-process monitoring (query /
//! get-by-id). Every resource here is participant-scoped and
//! version-segmented exactly like `IdentityHubClient`, so this is a
//! dedicated struct with the same `endpoint` + `version` + `bearer_token`
//! shape rather than an extension of `IssuerServiceClient` (which fronts
//! only the unauthenticated, unversioned `/metadata` endpoint and whose
//! `get_metadata()` signature other consumers may already rely on).
//!
//! Unlike `IdentityHubClient`, which hardcodes its `/api/identity` prefix,
//! this client takes the admin-api's base path (`admin_api_path`) as an
//! explicit constructor parameter -- callers pass in
//! `Config::issuer_admin_api_path` (or equivalent) so the prefix stays
//! configurable per deployment instead of being baked in as a literal.

use crate::errors::{IdentityHubClientError, Result};
use crate::models::{
  CredentialDefinition, CredentialDefinitionDto, CredentialOfferDto, CredentialStatusResponse,
  Holder, HolderDto, IssuanceProcessDto, QuerySpec, VerifiableCredentialResourceDto,
};
use crate::IdentityHubClientVersion;

pub struct IssuerAdminApiClient {
  client: reqwest::Client,
  endpoint: String,
  admin_api_path: String,
  bearer_token: Option<String>,
  version: IdentityHubClientVersion,
}

impl IssuerAdminApiClient {
  pub fn new(
    client: reqwest::Client,
    endpoint: String,
    admin_api_path: String,
    bearer_token: Option<String>,
    version: IdentityHubClientVersion,
  ) -> Self {
    Self {
      client,
      endpoint,
      admin_api_path,
      bearer_token,
      version,
    }
  }

  /// Base URL for a given participant-scoped resource collection, e.g.
  /// `{endpoint}{admin_api_path}/{version}/participants/{id}/holders`.
  fn resource_url(&self, participant_context_id: &str, resource: &str) -> String {
    format!(
      "{}{}/{}/participants/{participant_context_id}/{resource}",
      self.endpoint, self.admin_api_path, self.version
    )
  }

  fn authorize(&self, request_builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
    if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    }
  }

  // ---------------------------------------------------------------------
  // credential definitions
  // ---------------------------------------------------------------------

  pub async fn create_credential_definition(
    &self,
    participant_context_id: &str,
    credential_definition: &CredentialDefinitionDto,
  ) -> Result<()> {
    let url = self.resource_url(participant_context_id, "credentialdefinitions");
    let request_builder = self.authorize(self.client.post(&url));

    let response = request_builder.json(credential_definition).send().await?;

    if response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn update_credential_definition(
    &self,
    participant_context_id: &str,
    credential_definition: &CredentialDefinitionDto,
  ) -> Result<()> {
    let url = self.resource_url(participant_context_id, "credentialdefinitions");
    let request_builder = self.authorize(self.client.put(&url));

    let response = request_builder.json(credential_definition).send().await?;

    if response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn get_credential_definition_by_id(
    &self,
    participant_context_id: &str,
    credential_definition_id: &str,
  ) -> Result<CredentialDefinition> {
    let url = format!(
      "{}/{credential_definition_id}",
      self.resource_url(participant_context_id, "credentialdefinitions")
    );
    let request_builder = self.authorize(self.client.get(&url));

    let response = request_builder.send().await?;

    if response.status().is_success() {
      Ok(response.json().await?)
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn query_credential_definitions(
    &self,
    participant_context_id: &str,
    query: &QuerySpec,
  ) -> Result<Vec<CredentialDefinition>> {
    let url = format!(
      "{}/query",
      self.resource_url(participant_context_id, "credentialdefinitions")
    );
    let request_builder = self.authorize(self.client.post(&url));

    let response = request_builder.json(query).send().await?;

    if response.status().is_success() {
      Ok(response.json().await?)
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn delete_credential_definition_by_id(
    &self,
    participant_context_id: &str,
    credential_definition_id: &str,
  ) -> Result<()> {
    let url = format!(
      "{}/{credential_definition_id}",
      self.resource_url(participant_context_id, "credentialdefinitions")
    );
    let request_builder = self.authorize(self.client.delete(&url));

    let response = request_builder.send().await?;

    if response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  // ---------------------------------------------------------------------
  // holders
  // ---------------------------------------------------------------------

  pub async fn create_holder(&self, participant_context_id: &str, holder: &HolderDto) -> Result<()> {
    let url = self.resource_url(participant_context_id, "holders");
    let request_builder = self.authorize(self.client.post(&url));

    let response = request_builder.json(holder).send().await?;

    if response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn get_holder(&self, participant_context_id: &str, holder_id: &str) -> Result<Holder> {
    let url = format!(
      "{}/{holder_id}",
      self.resource_url(participant_context_id, "holders")
    );
    let request_builder = self.authorize(self.client.get(&url));

    let response = request_builder.send().await?;

    if response.status().is_success() {
      Ok(response.json().await?)
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn query_holders(
    &self,
    participant_context_id: &str,
    query: &QuerySpec,
  ) -> Result<Vec<Holder>> {
    let url = format!(
      "{}/query",
      self.resource_url(participant_context_id, "holders")
    );
    let request_builder = self.authorize(self.client.post(&url));

    let response = request_builder.json(query).send().await?;

    if response.status().is_success() {
      Ok(response.json().await?)
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn delete_holder(&self, participant_context_id: &str, holder_id: &str) -> Result<()> {
    let url = format!(
      "{}/{holder_id}",
      self.resource_url(participant_context_id, "holders")
    );
    let request_builder = self.authorize(self.client.delete(&url));

    let response = request_builder.send().await?;

    if response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn update_holder(&self, participant_context_id: &str, holder: &HolderDto) -> Result<()> {
    let url = self.resource_url(participant_context_id, "holders");
    let request_builder = self.authorize(self.client.put(&url));

    let response = request_builder.json(holder).send().await?;

    if response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  // ---------------------------------------------------------------------
  // credentials / issuance
  // ---------------------------------------------------------------------

  pub async fn query_credentials(
    &self,
    participant_context_id: &str,
    query: &QuerySpec,
  ) -> Result<Vec<VerifiableCredentialResourceDto>> {
    let url = format!(
      "{}/query",
      self.resource_url(participant_context_id, "credentials")
    );
    let request_builder = self.authorize(self.client.post(&url));

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
      "{}/{credential_id}/status",
      self.resource_url(participant_context_id, "credentials")
    );
    let request_builder = self.authorize(self.client.get(&url));

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
      "{}/{credential_id}/revoke",
      self.resource_url(participant_context_id, "credentials")
    );
    let request_builder = self.authorize(self.client.post(&url));

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
      "{}/{credential_id}/suspend",
      self.resource_url(participant_context_id, "credentials")
    );
    let request_builder = self.authorize(self.client.post(&url));

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
      "{}/{credential_id}/resume",
      self.resource_url(participant_context_id, "credentials")
    );
    let request_builder = self.authorize(self.client.post(&url));

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
      "{}/offer",
      self.resource_url(participant_context_id, "credentials")
    );
    let request_builder = self.authorize(self.client.post(&url));

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
      "{}/query",
      self.resource_url(participant_context_id, "issuanceprocesses")
    );
    let request_builder = self.authorize(self.client.post(&url));

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
      "{}/{issuance_process_id}",
      self.resource_url(participant_context_id, "issuanceprocesses")
    );
    let request_builder = self.authorize(self.client.get(&url));

    let response = request_builder.send().await?;

    if response.status().is_success() {
      Ok(response.json().await?)
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }
}

// wiremock/tokio are only pulled in as dev-dependencies for non-wasm32
// targets (they run a real hyper server), so these contract tests must be
// gated the same way -- otherwise `cargo check --target
// wasm32-unknown-unknown --all-targets` fails trying to resolve `wiremock`.
#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
  use super::*;
  use serde_json::json;
  use wiremock::matchers::{body_json, header, method, path};
  use wiremock::{Mock, MockServer, ResponseTemplate};

  const ADMIN_API_PATH: &str = "/api/issuer";

  fn client_with_token(endpoint: String, token: &str) -> IssuerAdminApiClient {
    IssuerAdminApiClient::new(
      reqwest::Client::new(),
      endpoint,
      ADMIN_API_PATH.to_string(),
      Some(token.to_string()),
      IdentityHubClientVersion::V1Beta,
    )
  }

  fn client(endpoint: String) -> IssuerAdminApiClient {
    IssuerAdminApiClient::new(
      reqwest::Client::new(),
      endpoint,
      ADMIN_API_PATH.to_string(),
      None,
      IdentityHubClientVersion::V1Beta,
    )
  }

  // -- credential definitions ------------------------------------------

  fn credential_definition_dto() -> CredentialDefinitionDto {
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

  fn credential_definition() -> CredentialDefinition {
    CredentialDefinition {
      id: "cred-def-1".to_string(),
      participant_context_id: "participant-1".to_string(),
      credential_type: "MembershipCredential".to_string(),
      format: "VC1_0_JWT".to_string(),
      json_schema: Some("{\"type\":\"object\"}".to_string()),
      json_schema_url: None,
      validity: 3600,
      attestations: vec!["attestation-1".to_string()],
      additional_context: vec![],
      rules: vec![],
      mappings: vec![],
    }
  }

  #[tokio::test]
  async fn create_credential_definition_posts_to_credentialdefinitions() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
      .and(path(
        "/api/issuer/v1beta/participants/participant-1/credentialdefinitions",
      ))
      .and(header("Authorization", "Bearer test-token"))
      .and(body_json(&credential_definition_dto()))
      .respond_with(ResponseTemplate::new(201))
      .mount(&server)
      .await;

    let client = client_with_token(server.uri(), "test-token");

    client
      .create_credential_definition("participant-1", &credential_definition_dto())
      .await
      .expect("create_credential_definition should succeed");
  }

  #[tokio::test]
  async fn update_credential_definition_puts_to_credentialdefinitions() {
    let server = MockServer::start().await;

    Mock::given(method("PUT"))
      .and(path(
        "/api/issuer/v1beta/participants/participant-1/credentialdefinitions",
      ))
      .and(body_json(&credential_definition_dto()))
      .respond_with(ResponseTemplate::new(200))
      .mount(&server)
      .await;

    let client = client(server.uri());

    client
      .update_credential_definition("participant-1", &credential_definition_dto())
      .await
      .expect("update_credential_definition should succeed");
  }

  #[tokio::test]
  async fn get_credential_definition_by_id_gets_single_resource() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
      .and(path(
        "/api/issuer/v1beta/participants/participant-1/credentialdefinitions/cred-def-1",
      ))
      .respond_with(ResponseTemplate::new(200).set_body_json(credential_definition()))
      .mount(&server)
      .await;

    let client = client(server.uri());

    let result = client
      .get_credential_definition_by_id("participant-1", "cred-def-1")
      .await
      .expect("get_credential_definition_by_id should succeed");

    assert_eq!(result.id, "cred-def-1");
    assert_eq!(result.credential_type, "MembershipCredential");
  }

  #[tokio::test]
  async fn query_credential_definitions_posts_query_and_returns_collection() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
      .and(path(
        "/api/issuer/v1beta/participants/participant-1/credentialdefinitions/query",
      ))
      .and(body_json(&QuerySpec::default()))
      .respond_with(ResponseTemplate::new(200).set_body_json(vec![credential_definition()]))
      .mount(&server)
      .await;

    let client = client(server.uri());

    let result = client
      .query_credential_definitions("participant-1", &QuerySpec::default())
      .await
      .expect("query_credential_definitions should succeed");

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "cred-def-1");
  }

  #[tokio::test]
  async fn delete_credential_definition_by_id_deletes_single_resource() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
      .and(path(
        "/api/issuer/v1beta/participants/participant-1/credentialdefinitions/cred-def-1",
      ))
      .respond_with(ResponseTemplate::new(204))
      .mount(&server)
      .await;

    let client = client(server.uri());

    client
      .delete_credential_definition_by_id("participant-1", "cred-def-1")
      .await
      .expect("delete_credential_definition_by_id should succeed");
  }

  // -- holders -----------------------------------------------------------

  #[tokio::test]
  async fn create_holder_posts_holder_dto_to_the_holders_endpoint() {
    let mock_server = MockServer::start().await;
    let holder = HolderDto::new(
      "holder-1".to_string(),
      "did:web:example.com:holder-1".to_string(),
      "Alice".to_string(),
    );

    Mock::given(method("POST"))
      .and(path(
        "/api/issuer/v1beta/participants/participant-1/holders",
      ))
      .and(header("Authorization", "Bearer test-token"))
      .and(body_json(&holder))
      .respond_with(ResponseTemplate::new(201))
      .expect(1)
      .mount(&mock_server)
      .await;

    let client = client_with_token(mock_server.uri(), "test-token");

    client
      .create_holder("participant-1", &holder)
      .await
      .expect("create_holder should succeed against the mocked endpoint");
  }

  #[tokio::test]
  async fn get_holder_fetches_a_single_holder_by_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
      .and(path(
        "/api/issuer/v1beta/participants/participant-1/holders/holder-1",
      ))
      .and(header("Authorization", "Bearer test-token"))
      .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "holderId": "holder-1",
        "participantContextId": "participant-1",
        "did": "did:web:example.com:holder-1",
        "holderName": "Alice",
        "anonymous": false,
        "properties": {"tier": "gold"},
        "lastModifiedAt": 1_700_000_000_000_i64
      })))
      .expect(1)
      .mount(&mock_server)
      .await;

    let client = client_with_token(mock_server.uri(), "test-token");

    let holder = client
      .get_holder("participant-1", "holder-1")
      .await
      .expect("get_holder should succeed against the mocked endpoint");

    assert_eq!(holder.holder_id, "holder-1");
    assert_eq!(holder.did, "did:web:example.com:holder-1");
    assert_eq!(holder.holder_name, "Alice");
    assert!(!holder.anonymous);
  }

  #[tokio::test]
  async fn query_holders_posts_query_spec_and_returns_a_collection() {
    let mock_server = MockServer::start().await;
    let query = QuerySpec::default();

    Mock::given(method("POST"))
      .and(path(
        "/api/issuer/v1beta/participants/participant-1/holders/query",
      ))
      .and(header("Authorization", "Bearer test-token"))
      .and(body_json(&query))
      .respond_with(ResponseTemplate::new(200).set_body_json(json!([
        {
          "holderId": "holder-1",
          "participantContextId": "participant-1",
          "did": "did:web:example.com:holder-1",
          "holderName": "Alice",
          "anonymous": false,
          "properties": {},
          "lastModifiedAt": 1_700_000_000_000_i64
        },
        {
          "holderId": "holder-2",
          "participantContextId": "participant-1",
          "did": "did:web:example.com:holder-2",
          "holderName": "Bob",
          "anonymous": true,
          "properties": {},
          "lastModifiedAt": 1_700_000_001_000_i64
        }
      ])))
      .expect(1)
      .mount(&mock_server)
      .await;

    let client = client_with_token(mock_server.uri(), "test-token");

    let holders = client
      .query_holders("participant-1", &query)
      .await
      .expect("query_holders should succeed against the mocked endpoint");

    assert_eq!(holders.len(), 2);
    assert_eq!(holders[0].holder_id, "holder-1");
    assert_eq!(holders[1].holder_id, "holder-2");
  }

  #[tokio::test]
  async fn delete_holder_sends_a_delete_request_for_the_holder_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
      .and(path(
        "/api/issuer/v1beta/participants/participant-1/holders/holder-1",
      ))
      .and(header("Authorization", "Bearer test-token"))
      .respond_with(ResponseTemplate::new(204))
      .expect(1)
      .mount(&mock_server)
      .await;

    let client = client_with_token(mock_server.uri(), "test-token");

    client
      .delete_holder("participant-1", "holder-1")
      .await
      .expect("delete_holder should succeed against the mocked endpoint");
  }

  #[tokio::test]
  async fn update_holder_puts_holder_dto_to_the_holders_endpoint() {
    let mock_server = MockServer::start().await;
    let holder = HolderDto::new(
      "holder-1".to_string(),
      "did:web:example.com:holder-1".to_string(),
      "Alice Updated".to_string(),
    );

    Mock::given(method("PUT"))
      .and(path(
        "/api/issuer/v1beta/participants/participant-1/holders",
      ))
      .and(header("Authorization", "Bearer test-token"))
      .and(body_json(&holder))
      .respond_with(ResponseTemplate::new(200))
      .expect(1)
      .mount(&mock_server)
      .await;

    let client = client_with_token(mock_server.uri(), "test-token");

    client
      .update_holder("participant-1", &holder)
      .await
      .expect("update_holder should succeed against the mocked endpoint");
  }

  // -- credentials / issuance ---------------------------------------------

  #[tokio::test]
  async fn query_credentials_posts_query_spec_and_returns_matching_resources() {
    let mock_server = MockServer::start().await;
    let participant_context_id = "participant-1";
    let query = QuerySpec::none();

    let response_body = json!([
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

    let client = client(mock_server.uri());

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
      .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "credentialId": credential_id,
        "status": "active",
        "reason": null
      })))
      .mount(&mock_server)
      .await;

    let client = client(mock_server.uri());

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

    let client = client_with_token(mock_server.uri(), "test-token");

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

    let client = client(mock_server.uri());

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

    let client = client(mock_server.uri());

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

    let client = client(mock_server.uri());

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

    let response_body = json!([
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

    let client = client(mock_server.uri());

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
      .respond_with(ResponseTemplate::new(200).set_body_json(json!({
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

    let client = client(mock_server.uri());

    let process = client
      .get_issuance_process(participant_context_id, issuance_process_id)
      .await
      .expect("get_issuance_process should succeed");

    assert_eq!(process.id, issuance_process_id);
    assert_eq!(process.state, "APPROVED");
  }
}
