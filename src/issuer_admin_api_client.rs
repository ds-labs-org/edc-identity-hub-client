//! Client for the Issuer Service's issuer-admin-api: verifiable-credential
//! lifecycle (query / status / revoke / suspend / resume / offer) and
//! issuance-process monitoring (query / get-by-id). Every resource here is
//! participant-scoped and version-segmented exactly like `IdentityHubClient`,
//! so this is a dedicated struct with the same `endpoint` + `version` +
//! `bearer_token` shape rather than an extension of `IssuerServiceClient`
//! (which fronts only the unauthenticated, unversioned `/metadata` endpoint
//! and whose `get_metadata()` signature other consumers may already rely on).

#[cfg(test)]
mod tests {
  use crate::IdentityHubClientVersion;
  use crate::models::QuerySpec;
  use wiremock::matchers::{body_json, method, path};
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
}
