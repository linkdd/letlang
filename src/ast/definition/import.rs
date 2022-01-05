#[derive(Debug, Clone)]
pub struct ImportDefinition {
  pub module: String,
  pub alias: Option<String>,
}
