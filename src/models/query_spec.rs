use serde::{Deserialize, Serialize};

/// Mirrors EDC's `org.eclipse.edc.spi.query.QuerySpec`, the request body accepted by
/// every issuer-admin-api / identity-api `POST .../query` endpoint (including
/// `queryCredentialDefinitions`, which the controller augments server-side with a
/// `filterByParticipantContextId` criterion -- callers don't need to add that filter
/// themselves).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySpec {
  #[serde(default)]
  pub offset: u32,
  #[serde(default = "QuerySpec::default_limit")]
  pub limit: u32,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub filter_expression: Vec<Criterion>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sort_field: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sort_order: Option<SortOrder>,
}

impl QuerySpec {
  fn default_limit() -> u32 {
    50
  }
}

impl Default for QuerySpec {
  fn default() -> Self {
    Self {
      offset: 0,
      limit: Self::default_limit(),
      filter_expression: Vec::new(),
      sort_field: None,
      sort_order: None,
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Criterion {
  pub operand_left: serde_json::Value,
  pub operator: String,
  pub operand_right: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SortOrder {
  Asc,
  Desc,
}
