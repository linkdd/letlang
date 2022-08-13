use crate::ast::Node;

mod import;
mod effect;
mod class;
mod func;

mod proposition;

pub use self::{
  import::ImportStatement,
  effect::EffectDeclStatement,
  class::ClassDeclStatement,
  func::FuncDeclStatement,
  proposition::{Proposition, Constraint},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
  Import(ImportStatement),
  EffectDecl(EffectDeclStatement),
  ClassDecl(ClassDeclStatement),
  FuncDecl(FuncDeclStatement),
}

use crate::ast::{
  types::TypeRef,
  params::{TypeParam, ConsParam, CallParam},
};

impl Statement {
  pub fn import(path: Vec<String>, alias: Option<String>) -> Box<Self> {
    Box::new(Self::Import(ImportStatement { path, alias }))
  }

  pub fn effect(
    public: bool,
    symbol_name: String,
    call_params: Vec<Node<CallParam>>,
    return_type: Node<TypeRef>,
  ) -> Box<Self> {
    Box::new(Self::EffectDecl(EffectDeclStatement {
      public,
      symbol_name,
      call_params,
      return_type,
    }))
  }

  pub fn class(
    public: bool,
    symbol_name: String,
    type_params: Vec<Node<TypeParam>>,
    cons_param: Node<ConsParam>,
    constraints: Vec<Node<Proposition>>,
  ) -> Box<Self> {
    Box::new(Self::ClassDecl(ClassDeclStatement {
      public,
      symbol_name,
      type_params,
      cons_param,
      constraints,
    }))
  }

  pub fn function(
    public: bool,
    symbol_name: String,
    type_params: Vec<Node<TypeParam>>,
    call_params: Vec<Node<CallParam>>,
    return_type: Node<TypeRef>,
    body: Vec<Node<Proposition>>,
  ) -> Box<Self> {
    Box::new(Self::FuncDecl(FuncDeclStatement {
      public,
      symbol_name,
      type_params,
      call_params,
      return_type,
      body,
    }))
  }
}
