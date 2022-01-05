use crate::ast::{VariableNameNode, TypeConstraintNode};

#[derive(Debug, Clone)]
pub struct TypeDef {
  pub var_name: VariableNameNode,
  pub var_type: TypeConstraintNode,
}

pub type TypeDefNode = Box<TypeDef>;
