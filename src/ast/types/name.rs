use crate::ast::TypeParamNode;

#[derive(Debug, Clone)]
pub struct TypeName {
  pub name: String,
  pub params: Vec<TypeParamNode>,
}

pub type TypeNameNode = Box<TypeName>;
