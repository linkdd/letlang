use crate::{
  Node,
  types::TypeRef,
  params::{TypeParam, CallParam},
  statement::Proposition,
};

#[derive(Clone, Debug, PartialEq)]
pub struct FuncDeclStatement {
  pub public: bool,
  pub symbol_name: String,
  pub type_params: Vec<Node<TypeParam>>,
  pub call_params: Vec<Node<CallParam>>,
  pub return_type: Node<TypeRef>,
  pub body: Vec<Node<Proposition>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TailRecFuncDeclStatement {
  pub public: bool,
  pub symbol_name: String,
  pub type_params: Vec<Node<TypeParam>>,
  pub call_params: Vec<Node<CallParam>>,
  pub return_type: Node<TypeRef>,
  pub body: Vec<Node<Proposition>>,
}
