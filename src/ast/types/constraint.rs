use crate::ast::{TypeNameNode, TypeParamNode, LiteralNode};

#[derive(Debug, Clone)]
pub enum TypeConstraint {
  Name { val: TypeNameNode },
  Literal { val: LiteralNode },
  Tuple { items: Vec<TypeConstraintNode> },
  List { items: TypeConstraintNode },
  Set { items: TypeConstraintNode },
  Object { entries: Vec<TypeParamNode> },
  Function { params: Vec<TypeConstraintNode>, return_type: TypeConstraintNode },
  Or { lhs: TypeConstraintNode, rhs: TypeConstraintNode },
  And { lhs: TypeConstraintNode, rhs: TypeConstraintNode },
  Not { val: TypeConstraintNode },
}

pub type TypeConstraintNode = Box<TypeConstraint>;
