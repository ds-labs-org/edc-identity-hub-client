use serde::{Deserialize, Serialize};

/// Mirrors EDC's `org.eclipse.edc.spi.query.QuerySpec` — the request body
/// every `POST .../query` admin endpoint expects.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySpec {
  pub offset: u32,
  pub limit: u32,
  pub sort_order: SortOrder,
  pub sort_field: Option<String>,
  pub filter_expression: Vec<Criterion>,
}

impl Default for QuerySpec {
  fn default() -> Self {
    Self {
      offset: 0,
      limit: 50,
      sort_order: SortOrder::Asc,
      sort_field: None,
      filter_expression: Vec::new(),
    }
  }
}

impl QuerySpec {
  /// Equivalent to `QuerySpec.none()` on the Java side: default paging
  /// (offset 0, limit 50), no sort, no filters.
  pub fn none() -> Self {
    Self::default()
  }
}

/// Mirrors `org.eclipse.edc.spi.query.SortOrder`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SortOrder {
  Asc,
  Desc,
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
