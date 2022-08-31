use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  types::*,
  expression::*,
};


impl<'compiler> Generator<'compiler> {
  pub fn gen_typeref(&self, node: &Node<TypeRef>) -> CompilationResult<String> {
    match node.data.as_ref() {
      TypeRef::Value(type_val) => {
        let value_code = self.gen_literal(&type_val.val)?;
        Ok(format!("ValueType {{ llval: {} }}", value_code))
      },
      TypeRef::TypeName(type_name) => {
        let Symbol(path) = &type_name.symbol;
        let mut type_params_code = vec![];

        for node in type_name.type_params.iter() {
          type_params_code.push(self.gen_typeref(node)?);
        }

        todo!();
      },
      _ => {
        todo!();
      }
    }
  }
}
