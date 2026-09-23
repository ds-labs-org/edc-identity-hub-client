use crate::errors::{IdentityHubClientError, Result};
use crate::models::{CredentialDefinition, CredentialDefinitionDto, QuerySpec};
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

  pub async fn update_credential_definition(
    &self,
    participant_context_id: &str,
    credential_definition: &CredentialDefinitionDto,
  ) -> Result<()> {
    let url = format!(
      "{}/api/issuer/{}/participants/{participant_context_id}/credentialdefinitions",
      self.endpoint, self.version
    );
    let request_builder = self.client.put(&url);

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

  pub async fn get_credential_definition_by_id(
    &self,
    participant_context_id: &str,
    credential_definition_id: &str,
  ) -> Result<CredentialDefinition> {
    let url = format!(
      "{}/api/issuer/{}/participants/{participant_context_id}/credentialdefinitions/{credential_definition_id}",
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

  pub async fn query_credential_definitions(
    &self,
    participant_context_id: &str,
    query: &QuerySpec,
  ) -> Result<Vec<CredentialDefinition>> {
    let url = format!(
      "{}/api/issuer/{}/participants/{participant_context_id}/credentialdefinitions/query",
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

  pub async fn delete_credential_definition_by_id(
    &self,
    participant_context_id: &str,
    credential_definition_id: &str,
  ) -> Result<()> {
    let url = format!(
      "{}/api/issuer/{}/participants/{participant_context_id}/credentialdefinitions/{credential_definition_id}",
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
  async fn update_credential_definition_puts_to_credentialdefinitions() {
    let server = MockServer::start().await;

    Mock::given(method("PUT"))
      .and(path(
        "/api/issuer/v1beta/participants/participant-1/credentialdefinitions",
      ))
      .and(body_json(&dto()))
      .respond_with(ResponseTemplate::new(200))
      .mount(&server)
      .await;

    let client = IssuerAdminApiClient::new(
      reqwest::Client::new(),
      server.uri(),
      None,
      IdentityHubClientVersion::V1Beta,
    );

    client
      .update_credential_definition("participant-1", &dto())
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

    let client = IssuerAdminApiClient::new(
      reqwest::Client::new(),
      server.uri(),
      None,
      IdentityHubClientVersion::V1Beta,
    );

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

    let client = IssuerAdminApiClient::new(
      reqwest::Client::new(),
      server.uri(),
      None,
      IdentityHubClientVersion::V1Beta,
    );

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

    let client = IssuerAdminApiClient::new(
      reqwest::Client::new(),
      server.uri(),
      None,
      IdentityHubClientVersion::V1Beta,
    );

    client
      .delete_credential_definition_by_id("participant-1", "cred-def-1")
      .await
      .expect("delete_credential_definition_by_id should succeed");
  }
}
