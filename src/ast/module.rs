use crate::ast::DefinitionNode;

#[derive(Debug, Clone)]
pub struct Module {
  pub definitions: Vec<DefinitionNode>,
}

pub type ModuleNode = Box<Module>;
