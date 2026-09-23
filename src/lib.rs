mod dataspace_service_client;
mod errors;
mod issuer_admin_api_client;
mod issuer_service_client;
pub mod models;

use crate::models::{
  CreateParticipantResponse, DidWeb, Participant, ParticipantContext, RequestCredentialInformation,
};
pub use dataspace_service_client::DataspaceServiceClient;
pub use errors::*;
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
}
