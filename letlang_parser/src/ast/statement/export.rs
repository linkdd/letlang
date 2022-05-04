use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub struct ExportStatement {
  pub symbol_name: String,
}
