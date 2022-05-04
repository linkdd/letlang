use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub struct ImportStatement {
  pub path: String,
  pub alias: Option<String>,
}
