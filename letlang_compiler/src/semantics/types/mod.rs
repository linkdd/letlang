use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::{
  Node,
  types::TypeRef,
};


impl<V: Visitor> Model<V> {
  pub fn visit_typeref(&mut self, node: &Node<TypeRef>) -> CompilationResult<()> {
    let result = match node.data.as_ref() {
      TypeRef::Value(val) => {
        self.visit_typeref_val(val)
      },
      TypeRef::TypeName(type_name) => {
        self.visit_typeref_name(type_name)
      },
      TypeRef::StructDefinition(def) => {
        self.visit_typeref_struct(def)
      },
      TypeRef::TupleDefinition(def) => {
        self.visit_typeref_tuple(def)
      },
      TypeRef::FunctionSignature(signature) => {
        self.visit_typeref_signature(signature)
      },
      TypeRef::OneOf(sum_type) => {
        self.visit_typeref_oneof(sum_type)
      },
      TypeRef::AllOf(sum_type) => {
        self.visit_typeref_allof(sum_type)
      },
      TypeRef::Not(sum_type) => {
        self.visit_typeref_not(sum_type)
      }
    };

    Self::locate_error(result, &node.location)
  }
}

mod base;
mod containers;
mod function;
mod sum_types;
