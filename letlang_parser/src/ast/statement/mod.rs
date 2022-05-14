use crate::ast::Node;
use serde::Serialize;

mod import;
mod constant;
mod effect;
mod class;
mod func;

pub use self::{
  import::ImportStatement,
  constant::ConstDeclStatement,
  effect::EffectDeclStatement,
  class::ClassDeclStatement,
  func::FuncDeclStatement,
};

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub enum Statement {
  Import(ImportStatement),
  ConstDecl(ConstDeclStatement),
  EffectDecl(EffectDeclStatement),
  ClassDecl(ClassDeclStatement),
  FuncDecl(FuncDeclStatement),
}

use crate::ast::{
  expression::Expression,
  types::{TypeParam, TypeRef},
  class::ConsParam,
  funcs::CallParam,
};

impl Statement {
  pub fn import(path: String, alias: Option<String>) -> Box<Self> {
    Box::new(Self::Import(ImportStatement { path, alias}))
  }

  pub fn constant(
    public: bool,
    symbol_name: String,
    value: Node<Expression>
  ) -> Box<Self> {
    Box::new(Self::ConstDecl(ConstDeclStatement {
      public,
      symbol_name,
      value
    }))
  }

  pub fn effect(
    public: bool,
    symbol_name: String,
    type_params: Vec<Node<TypeParam>>,
    call_params: Vec<Node<CallParam>>,
    return_type: Node<TypeRef>,
  ) -> Box<Self> {
    Box::new(Self::EffectDecl(EffectDeclStatement {
      public,
      symbol_name,
      type_params,
      call_params,
      return_type,
    }))
  }

  pub fn class(
    public: bool,
    symbol_name: String,
    type_params: Vec<Node<TypeParam>>,
    cons_param: Node<ConsParam>,
    constraints: Vec<Node<Expression>>,
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
    body: Vec<Node<Expression>>,
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
