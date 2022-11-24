use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  types::*,
};


impl<'compiler> Generator<'compiler> {
  pub fn gen_typeref(&self, node: &Node<TypeRef>) -> CompilationResult<String> {
    let attrs = node.attrs.as_ref().unwrap();

    match node.data.as_ref() {
      TypeRef::Value(type_val) => {
        self.gen_typeref_value(type_val)
      },
      TypeRef::TypeName(type_name) => {
        self.gen_typeref_name(type_name, attrs)
      },
      TypeRef::TupleDefinition(tuple_def) => {
        self.gen_typeref_tuple(&node.location, tuple_def)
      },
      TypeRef::OneOf(oneof_typeref) => {
        self.gen_typeref_oneof(&node.location, oneof_typeref)
      },
      _ => {
        todo!();
      }
    }
  }
}

mod value;
mod name;
mod tuple;
mod oneof;
