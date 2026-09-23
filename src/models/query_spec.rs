use serde::{Deserialize, Serialize};

/// Mirrors EDC's generic `org.eclipse.edc.spi.query.QuerySpec`, sent as the
/// JSON body of a `POST .../query` request (used by the issuer-admin-api's
/// `holders/query` endpoint, among others).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySpec {
  pub offset: usize,
  pub limit: usize,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub filter_expression: Vec<Criterion>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub sort_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub sort_order: Option<SortOrder>,
}

impl Default for QuerySpec {
  /// EDC's own `QuerySpec.Builder` defaults to offset 0, limit 50.
  fn default() -> Self {
    Self {
      offset: 0,
      limit: 50,
      filter_expression: Vec::new(),
      sort_field: None,
      sort_order: None,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Criterion {
  pub operand_left: serde_json::Value,
  pub operator: String,
  pub operand_right: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SortOrder {
  Asc,
  Desc,
}
