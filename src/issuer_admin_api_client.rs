use crate::errors::{IdentityHubClientError, Result};
use crate::models::{Holder, HolderDto, QuerySpec};
use crate::IdentityHubClientVersion;

/// Client for the issuer-admin-api's participant-scoped, version-segmented
/// resources (credential definitions, holders, credentials,
/// issuance-processes). Kept separate from `IssuerServiceClient`, whose
/// `get_metadata()` talks to the unversioned, unscoped `/metadata` endpoint
/// and has different constructor needs.
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

  fn holders_url(&self, participant_context_id: &str) -> String {
    format!(
      "{}/api/issuer/{}/participants/{participant_context_id}/holders",
      self.endpoint, self.version
    )
  }

  pub async fn create_holder(
    &self,
    participant_context_id: &str,
    holder: &HolderDto,
  ) -> Result<()> {
    let url = self.holders_url(participant_context_id);
    let request_builder = self.client.post(&url);

    let request_builder = if let Some(bearer_token) = &self.bearer_token {
      request_builder.header("Authorization", format!("Bearer {bearer_token}"))
    } else {
      request_builder
    };

    let response = request_builder.json(holder).send().await?;

    if response.status().is_success() {
      Ok(())
    } else {
      Err(IdentityHubClientError::Response(response))
    }
  }

  pub async fn get_holder(&self, participant_context_id: &str, holder_id: &str) -> Result<Holder> {
    let url = format!("{}/{holder_id}", self.holders_url(participant_context_id));
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

  pub async fn query_holders(
    &self,
    participant_context_id: &str,
    query: &QuerySpec,
  ) -> Result<Vec<Holder>> {
    let url = format!("{}/query", self.holders_url(participant_context_id));
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

  pub async fn delete_holder(&self, participant_context_id: &str, holder_id: &str) -> Result<()> {
    let url = format!("{}/{holder_id}", self.holders_url(participant_context_id));
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
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
  use super::*;
  use serde_json::json;
  use wiremock::matchers::{body_json, header, method, path};
  use wiremock::{Mock, MockServer, ResponseTemplate};

  fn client(endpoint: String) -> IssuerAdminApiClient {
    IssuerAdminApiClient::new(
      reqwest::Client::new(),
      endpoint,
      Some("test-token".to_string()),
      IdentityHubClientVersion::V1Beta,
    )
  }

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

    let client = client(mock_server.uri());

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

    let client = client(mock_server.uri());

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

    let client = client(mock_server.uri());

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

    let client = client(mock_server.uri());

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

    let client = client(mock_server.uri());

    client
      .update_holder("participant-1", &holder)
      .await
      .expect("update_holder should succeed against the mocked endpoint");
  }
}
