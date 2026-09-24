mod dataspace_service_client;
mod errors;
mod issuer_admin_api_client;
mod issuer_service_client;
pub mod models;

use crate::models::{
  CreateParticipantResponse, DidDocument, DidRequestPayload, DidWeb, KeyDescriptor, KeyPairResource,
  Participant, ParticipantContext, QuerySpec, RequestCredentialInformation,
};
pub use dataspace_service_client::DataspaceServiceClient;
pub use errors::*;
pub use issuer_admin_api_client::IssuerAdminApiClient;
pub use issuer_service_client::IssuerServiceClient;
use std::fmt::Display;

pub enum IdentityHubClientVersion {
  V1Alpha,
  V1Beta,
}

impl Display for IdentityHubClientVersion {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      IdentityHubClientVersion::V1Alpha => write!(f, "v1alpha"),
      IdentityHubClientVersion::V1Beta => write!(f, "v1beta"),
    }
  }
}

#[cfg(test)]
mod version_tests {
  use super::*;

  // EDC IdentityHub v0.18.0's identity-api and issuer-admin-api both moved
  // their UNSTABLE version segment from /v1alpha to /v1beta (confirmed
  // against org.eclipse.edc.identityhub.api.Versions.UNSTABLE in the real
  // source tree). Both variants are kept: v1alpha for whatever EDC version
  // this crate's other existing consumers still run.
  #[test]
  fn v1alpha_displays_as_v1alpha() {
    assert_eq!(IdentityHubClientVersion::V1Alpha.to_string(), "v1alpha");
  }

  #[test]
  fn v1beta_displays_as_v1beta() {
    assert_eq!(IdentityHubClientVersion::V1Beta.to_string(), "v1beta");
  }
}

pub struct IdentityHubClient {
  client: reqwest::Client,
  endpoint: String,
  bearer_token: Option<String>,
  version: IdentityHubClientVersion,
}

impl IdentityHubClient {
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

  pub async fn get_identity(
    client: reqwest::Client,
    participant: DidWeb,
  ) -> Result<models::Identity> {
    let request_builder = client.get(participant.url());

    let response = request_builder.send().await?;

    Ok(response.json().await?)
  }

  pub async fn create_participant(
    &self,
    participant_context: &ParticipantContext,
  ) -> Result<CreateParticipantResponse> {
    let url = format!(
      "{}/api/identity/{}/participants",
      self.endpoint, self.version
    );
    let request_builder = self.client.post(&url);

    let request_builder = if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    };

    let response = request_builder.json(&participant_context).send().await?;

    if response.status().is_success() {
      Ok(response.json::<CreateParticipantResponse>().await?)
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn get_participants(&self, offset: usize, limit: usize) -> Result<Vec<Participant>> {
    let url = format!(
      "{}/api/identity/{}/participants?offset={offset}&limit={limit}",
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

  pub async fn get_participant(&self, participant_context_id: &str) -> Result<Participant> {
    let url = format!(
      "{}/api/identity/{}/participants/{participant_context_id}",
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

  pub async fn activate_participant(
    &self,
    participant_context_id: &str,
    is_active: bool,
  ) -> Result<()> {
    let url = format!(
      "{}/api/identity/{}/participants/{participant_context_id}/state?isActive={is_active}",
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

  pub async fn delete_participant(&self, participant_context_id: &str) -> Result<Participant> {
    let url = format!(
      "{}/api/identity/{}/participants/{participant_context_id}",
      self.endpoint, self.version
    );
    let request_builder = self.client.delete(&url);

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

  pub async fn get_credentials(&self, participant_id: &str) -> Result<Vec<models::Credential>> {
    let url = format!(
      "{}/api/identity/{}/participants/{participant_id}/credentials",
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

  pub async fn get_credential(
    &self,
    participant_id: &str,
    credential_id: &str,
  ) -> Result<models::Credential> {
    let url = format!(
      "{}/api/identity/{}/participants/{participant_id}/credentials/{credential_id}",
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

  pub async fn delete_credential(&self, participant_id: &str, credential_id: &str) -> Result<()> {
    let url = format!(
      "{}/api/identity/{}/participants/{participant_id}/credentials/{credential_id}",
      self.endpoint, self.version
    );
    let request_builder = self.client.delete(&url);

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

  pub async fn request_verifiable_credential(
    &self,
    participant_id: &str,
    request_credential_body: &models::RequestCredentialBody,
  ) -> Result<()> {
    let url = format!(
      "{}/api/identity/{}/participants/{participant_id}/credentials/request",
      self.endpoint, self.version
    );
    let request_builder = self.client.post(&url);

    let request_builder = if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    };

    let response = request_builder.json(request_credential_body).send().await?;

    if !response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn get_request_verifiable_credential_status(
    &self,
    participant_id: &str,
    holder_pid: &str,
  ) -> Result<RequestCredentialInformation> {
    let url = format!(
      "{}/api/identity/{}/participants/{participant_id}/credentials/request/{holder_pid}",
      self.endpoint, self.version
    );

    let request_builder = self.client.get(&url);

    let request_builder = if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    };

    Ok(request_builder.send().await?.json().await?)
  }

  // ---------------------------------------------------------------------
  // DID management (.../dids)
  // ---------------------------------------------------------------------

  pub async fn publish_did(&self, participant_context_id: &str, did: &str) -> Result<()> {
    let url = format!(
      "{}/api/identity/{}/participants/{participant_context_id}/dids/publish",
      self.endpoint, self.version
    );
    let request_builder = self.client.post(&url);

    let request_builder = if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    };

    let response = request_builder
      .json(&DidRequestPayload::new(did))
      .send()
      .await?;

    if response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn unpublish_did(&self, participant_context_id: &str, did: &str) -> Result<()> {
    let url = format!(
      "{}/api/identity/{}/participants/{participant_context_id}/dids/unpublish",
      self.endpoint, self.version
    );
    let request_builder = self.client.post(&url);

    let request_builder = if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    };

    let response = request_builder
      .json(&DidRequestPayload::new(did))
      .send()
      .await?;

    if response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn query_dids(
    &self,
    participant_context_id: &str,
    query: &QuerySpec,
  ) -> Result<Vec<DidDocument>> {
    let url = format!(
      "{}/api/identity/{}/participants/{participant_context_id}/dids/query",
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

  /// `POST .../dids/state` answers with the `DidState` name as its body
  /// (e.g. `PUBLISHED`) -- some deployments serialize it as a raw
  /// text/plain string and others as a quoted JSON string, so surrounding
  /// quotes are stripped defensively either way.
  pub async fn get_did_state(&self, participant_context_id: &str, did: &str) -> Result<String> {
    let url = format!(
      "{}/api/identity/{}/participants/{participant_context_id}/dids/state",
      self.endpoint, self.version
    );
    let request_builder = self.client.post(&url);

    let request_builder = if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    };

    let response = request_builder
      .json(&DidRequestPayload::new(did))
      .send()
      .await?;

    if response.status().is_success() {
      let body = response.text().await?;
      Ok(body.trim().trim_matches('"').to_string())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  // ---------------------------------------------------------------------
  // Keypair management (.../keypairs)
  // ---------------------------------------------------------------------

  pub async fn list_keypairs(&self, participant_context_id: &str) -> Result<Vec<KeyPairResource>> {
    let url = format!(
      "{}/api/identity/{}/participants/{participant_context_id}/keypairs",
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

  pub async fn get_keypair(
    &self,
    participant_context_id: &str,
    key_pair_id: &str,
  ) -> Result<KeyPairResource> {
    let url = format!(
      "{}/api/identity/{}/participants/{participant_context_id}/keypairs/{key_pair_id}",
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

  pub async fn add_keypair(
    &self,
    _participant_context_id: &str,
    _descriptor: &KeyDescriptor,
    _make_default: bool,
  ) -> Result<()> {
    unimplemented!("add_keypair")
  }
}

// wiremock/tokio are only pulled in as dev-dependencies for non-wasm32
// targets (they run a real hyper server), so these contract tests must be
// gated the same way -- otherwise `cargo check --target
// wasm32-unknown-unknown --all-targets` fails trying to resolve `wiremock`.
#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod identity_hub_client_tests {
  use super::*;
  use wiremock::matchers::{body_json, header, method, path};
  use wiremock::{Mock, MockServer, ResponseTemplate};

  fn client_with_token(endpoint: String, token: &str) -> IdentityHubClient {
    IdentityHubClient::new(
      reqwest::Client::new(),
      endpoint,
      Some(token.to_string()),
      IdentityHubClientVersion::V1Beta,
    )
  }

  #[allow(dead_code)]
  fn client(endpoint: String) -> IdentityHubClient {
    IdentityHubClient::new(
      reqwest::Client::new(),
      endpoint,
      None,
      IdentityHubClientVersion::V1Beta,
    )
  }

  // -- dids ------------------------------------------------------------

  #[tokio::test]
  async fn publish_did_posts_to_dids_publish() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
      .and(path(
        "/api/identity/v1beta/participants/participant-1/dids/publish",
      ))
      .and(header("Authorization", "Bearer test-token"))
      .and(body_json(&DidRequestPayload::new("did:web:example.com")))
      .respond_with(ResponseTemplate::new(204))
      .expect(1)
      .mount(&server)
      .await;

    let client = client_with_token(server.uri(), "test-token");

    client
      .publish_did("participant-1", "did:web:example.com")
      .await
      .expect("publish_did should succeed against the mocked endpoint");
  }

  #[tokio::test]
  async fn unpublish_did_posts_to_dids_unpublish() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
      .and(path(
        "/api/identity/v1beta/participants/participant-1/dids/unpublish",
      ))
      .and(header("Authorization", "Bearer test-token"))
      .and(body_json(&DidRequestPayload::new("did:web:example.com")))
      .respond_with(ResponseTemplate::new(204))
      .expect(1)
      .mount(&server)
      .await;

    let client = client_with_token(server.uri(), "test-token");

    client
      .unpublish_did("participant-1", "did:web:example.com")
      .await
      .expect("unpublish_did should succeed against the mocked endpoint");
  }

  fn did_document() -> DidDocument {
    DidDocument {
      id: "did:web:example.com".to_string(),
      context: vec![serde_json::json!("https://www.w3.org/ns/did/v1")],
      service: vec![],
      verification_method: vec![],
      authentication: vec![],
      capability_invocation: vec![],
    }
  }

  #[tokio::test]
  async fn query_dids_posts_query_and_returns_collection() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
      .and(path(
        "/api/identity/v1beta/participants/participant-1/dids/query",
      ))
      .and(header("Authorization", "Bearer test-token"))
      .and(body_json(&QuerySpec::default()))
      .respond_with(ResponseTemplate::new(200).set_body_json(vec![did_document()]))
      .expect(1)
      .mount(&server)
      .await;

    let client = client_with_token(server.uri(), "test-token");

    let result = client
      .query_dids("participant-1", &QuerySpec::default())
      .await
      .expect("query_dids should succeed against the mocked endpoint");

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "did:web:example.com");
  }

  #[tokio::test]
  async fn get_did_state_posts_did_and_returns_plain_text_state() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
      .and(path(
        "/api/identity/v1beta/participants/participant-1/dids/state",
      ))
      .and(header("Authorization", "Bearer test-token"))
      .and(body_json(&DidRequestPayload::new("did:web:example.com")))
      .respond_with(ResponseTemplate::new(200).set_body_string("PUBLISHED"))
      .expect(1)
      .mount(&server)
      .await;

    let client = client_with_token(server.uri(), "test-token");

    let state = client
      .get_did_state("participant-1", "did:web:example.com")
      .await
      .expect("get_did_state should succeed against the mocked endpoint");

    assert_eq!(state, "PUBLISHED");
  }

  // -- keypairs ----------------------------------------------------------

  fn key_pair_resource(id: &str, default_pair: bool) -> KeyPairResource {
    KeyPairResource {
      id: id.to_string(),
      created_at: 1_700_000_000_000,
      participant_context_id: "participant-1".to_string(),
      timestamp: 1_700_000_000_000,
      key_id: format!("{id}-kid"),
      group_name: None,
      key_context: Some("JsonWebKey2020".to_string()),
      default_pair,
      use_duration: 15_552_000_000,
      rotation_duration: 0,
      serialized_public_key: "{\"kty\":\"EC\"}".to_string(),
      private_key_alias: format!("{id}-alias"),
      state: 200,
      usage: vec![crate::models::KeyPairUsage::SignCredentials],
    }
  }

  #[tokio::test]
  async fn list_keypairs_gets_the_keypairs_collection() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
      .and(path(
        "/api/identity/v1beta/participants/participant-1/keypairs",
      ))
      .and(header("Authorization", "Bearer test-token"))
      .respond_with(
        ResponseTemplate::new(200).set_body_json(vec![key_pair_resource("keypair-1", true)]),
      )
      .expect(1)
      .mount(&server)
      .await;

    let client = client_with_token(server.uri(), "test-token");

    let keypairs = client
      .list_keypairs("participant-1")
      .await
      .expect("list_keypairs should succeed against the mocked endpoint");

    assert_eq!(keypairs.len(), 1);
    assert_eq!(keypairs[0].id, "keypair-1");
    assert!(keypairs[0].default_pair);
  }

  #[tokio::test]
  async fn get_keypair_gets_a_single_resource_by_id() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
      .and(path(
        "/api/identity/v1beta/participants/participant-1/keypairs/keypair-1",
      ))
      .and(header("Authorization", "Bearer test-token"))
      .respond_with(ResponseTemplate::new(200).set_body_json(key_pair_resource("keypair-1", true)))
      .expect(1)
      .mount(&server)
      .await;

    let client = client_with_token(server.uri(), "test-token");

    let keypair = client
      .get_keypair("participant-1", "keypair-1")
      .await
      .expect("get_keypair should succeed against the mocked endpoint");

    assert_eq!(keypair.id, "keypair-1");
    assert_eq!(keypair.state, 200);
  }

  fn key_generator_descriptor() -> KeyDescriptor {
    KeyDescriptor {
      key_id: Some("bootstrap-key-1".to_string()),
      key_generator_params: Some(serde_json::json!({"algorithm": "EC"})),
      ..Default::default()
    }
  }

  #[tokio::test]
  async fn add_keypair_puts_descriptor_with_make_default_query_param() {
    let server = MockServer::start().await;

    Mock::given(method("PUT"))
      .and(path(
        "/api/identity/v1beta/participants/participant-1/keypairs",
      ))
      .and(wiremock::matchers::query_param("makeDefault", "true"))
      .and(header("Authorization", "Bearer test-token"))
      .and(body_json(&key_generator_descriptor()))
      .respond_with(ResponseTemplate::new(201))
      .expect(1)
      .mount(&server)
      .await;

    let client = client_with_token(server.uri(), "test-token");

    client
      .add_keypair("participant-1", &key_generator_descriptor(), true)
      .await
      .expect("add_keypair should succeed against the mocked endpoint");
  }
}
