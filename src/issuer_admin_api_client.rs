use crate::errors::{IdentityHubClientError, Result};
use crate::models::{Holder, HolderDto};
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
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
  use super::*;
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
}
