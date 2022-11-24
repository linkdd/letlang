use crate::{
  Node,
  params::{TypeParam, ConsParam},
  statement::Proposition,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ClassDeclStatement {
  pub public: bool,
  pub symbol_name: String,
  pub type_params: Vec<Node<TypeParam>>,
  pub cons_param: Node<ConsParam>,
  pub constraints: Vec<Node<Proposition>>,
}
