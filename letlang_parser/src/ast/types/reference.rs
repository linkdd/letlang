use crate::ast::{
  Node,
  expression::Literal,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeRef {
  TypeName { path: Vec<String>, type_params: Vec<Node<TypeRef>> },
  StructDefinition {
    members: Vec<(String, Node<TypeRef>)>,
  },
  TupleDefinition {
    members: Vec<Node<TypeRef>>,
  },
  FunctionSignature { params: Vec<Node<TypeRef>>, return_type: Node<TypeRef> },
  Value(Node<Literal>),
  OneOf(Vec<Node<TypeRef>>),
  AllOf(Vec<Node<TypeRef>>),
  Not(Node<TypeRef>),
}

impl TypeRef {
  pub fn type_name(path: Vec<String>, type_params: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::TypeName { path, type_params })
  }

  pub fn struct_definition(members: Vec<(String, Node<TypeRef>)>) -> Box<Self> {
    Box::new(Self::StructDefinition { members })
  }

  pub fn tuple_definition(members: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::TupleDefinition { members })
  }

  pub fn function_signature(params: Vec<Node<TypeRef>>, return_type: Node<TypeRef>) -> Box<Self> {
    Box::new(Self::FunctionSignature { params, return_type })
  }

  pub fn value(val: Node<Literal>) -> Box<Self> {
    Box::new(Self::Value(val))
  }

  pub fn one_of(typerefs: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::OneOf(typerefs))
  }

  pub fn all_of(typerefs: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::AllOf(typerefs))
  }

  pub fn not(typeref: Node<TypeRef>) -> Box<Self> {
    Box::new(Self::Not(typeref))
  }
}
