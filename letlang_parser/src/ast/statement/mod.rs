use crate::ast::Node;
use serde::Serialize;

mod import;
mod export;
mod constant;
mod effect;
mod class;
mod func;

pub use self::{
  import::ImportStatement,
  export::ExportStatement,
  constant::ConstDeclStatement,
  effect::EffectDeclStatement,
  class::ClassDeclStatement,
  func::FuncDeclStatement,
};

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub enum Statement {
  Import(ImportStatement),
  Export(ExportStatement),
  ConstDecl(ConstDeclStatement),
  EffectDecl(EffectDeclStatement),
  ClassDecl(ClassDeclStatement),
  FuncDecl(FuncDeclStatement),
}

use crate::ast::{
  expression::Expression,
  types::{TypeParam, TypeRef},
  class::{ConsParam, Formula},
  funcs::CallParam,
};

impl Statement {
  pub fn import(path: String, alias: Option<String>) -> Box<Self> {
    Box::new(Self::Import(ImportStatement { path, alias}))
  }

  pub fn export(symbol_name: String) -> Box<Self> {
    Box::new(Self::Export(ExportStatement { symbol_name }))
  }

  pub fn constant(symbol_name: String, value: Node<Expression>) -> Box<Self> {
    Box::new(Self::ConstDecl(ConstDeclStatement { symbol_name, value }))
  }

  pub fn effect(
    symbol_name: String,
    type_params: Vec<Node<TypeParam>>,
    call_params: Vec<Node<CallParam>>,
    return_type: Node<TypeRef>,
  ) -> Box<Self> {
    Box::new(Self::EffectDecl(EffectDeclStatement {
      symbol_name,
      type_params,
      call_params,
      return_type,
    }))
  }

  pub fn class(
    symbol_name: String,
    type_params: Vec<Node<TypeParam>>,
    cons_params: Vec<Node<ConsParam>>,
    constraints: Node<Formula>,
  ) -> Box<Self> {
    Box::new(Self::ClassDecl(ClassDeclStatement {
      symbol_name,
      type_params,
      cons_params,
      constraints,
    }))
  }

  pub fn function(
    symbol_name: String,
    type_params: Vec<Node<TypeParam>>,
    call_params: Vec<Node<CallParam>>,
    return_type: Node<TypeRef>,
    body: Vec<Node<Expression>>,
  ) -> Box<Self> {
    Box::new(Self::FuncDecl(FuncDeclStatement {
      symbol_name,
      type_params,
      call_params,
      return_type,
      body,
    }))
  }
}
