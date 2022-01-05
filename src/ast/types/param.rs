use crate::ast::TypeConstraintNode;

#[derive(Debug, Clone)]
pub struct TypeParam {
  pub name: String,
  pub constraint: TypeConstraintNode,
}

pub type TypeParamNode = Box<TypeParam>;
