#[derive(Debug, Clone)]
pub struct VariableName {
  pub name: String,
}

pub type VariableNameNode = Box<VariableName>;
