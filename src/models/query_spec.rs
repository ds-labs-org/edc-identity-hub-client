use serde::{Deserialize, Serialize};

/// Mirrors EDC's `org.eclipse.edc.spi.query.QuerySpec`, the request body accepted by
/// every issuer-admin-api / identity-api `POST .../query` endpoint (including
/// `queryCredentialDefinitions`, which the controller augments server-side with a
/// `filterByParticipantContextId` criterion -- callers don't need to add that filter
/// themselves).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySpec {
  #[serde(default)]
  pub offset: u32,
  #[serde(default = "QuerySpec::default_limit")]
  pub limit: u32,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub filter_expression: Vec<Criterion>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub sort_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub sort_order: Option<SortOrder>,
}

impl QuerySpec {
  fn default_limit() -> u32 {
    50
  }

  /// Equivalent to `QuerySpec.none()` on the Java side: default paging
  /// (offset 0, limit 50), no sort, no filters.
  pub fn none() -> Self {
    Self::default()
  }
}

impl Default for QuerySpec {
  /// EDC's own `QuerySpec.Builder` defaults to offset 0, limit 50.
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

/// Mirrors `org.eclipse.edc.spi.query.Criterion` — a single `left operator
/// right` filter expression, e.g. `participantContextId = someId`.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Criterion {
  pub operand_left: serde_json::Value,
  pub operator: String,
  pub operand_right: Option<serde_json::Value>,
}

impl Criterion {
  pub fn new(
    operand_left: impl Into<serde_json::Value>,
    operator: impl Into<String>,
    operand_right: Option<serde_json::Value>,
  ) -> Self {
    Self {
      operand_left: operand_left.into(),
      operator: operator.into(),
      operand_right,
    }
  }
}

/// Mirrors `org.eclipse.edc.spi.query.SortOrder`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SortOrder {
  Asc,
  Desc,
}
