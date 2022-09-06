use crate::prelude::*;
use crate::scope::*;
pub use super::Generator;

use letlang_ast::{
  *,
  types::*,
  expression::*,
};


impl<'compiler> Generator<'compiler> {
  pub fn gen_typeref_name(
    &self,
    data: &TypeName,
    attrs: &TypeRefAttributes,
  ) -> CompilationResult<String> {
    let type_scope = self.get_symbol_scope(
      data.symbol.scope(),
      attrs.scope_id
    );

    let (_, type_sym) = type_scope.lookup_symbol(
      self.scope_arena,
      &data.symbol.name(),
    ).unwrap();

    match type_sym {
      SymbolKind::Class { type_param_count, builtin: true } => {
        Ok(self.gen_builtin_class(data, type_param_count))
      },
      SymbolKind::Class { type_param_count, builtin: false } => {
        todo!();
      },
      SymbolKind::TypeParameter { index } => {
        todo!();
      },
      _ => {
        unreachable!("\
          Semantic validation of the AST should have proven \
          that this symbol is either a class or a type parameter\
        ");
      }
    }
  }

  fn gen_builtin_class(
    &self,
    data: &TypeName,
    type_param_count: usize,
  ) -> String {
    match data.symbol.name().as_str() {
      "boolean" => {
        "BooleanType {}".to_string()
      },
      "number" => {
        "NumberType {}".to_string()
      },
      "int" => {
        "IntegerType {}".to_string()
      },
      "string" => {
        "StringType {}".to_string()
      },
      "atom" => {
        "AtomType {}".to_string()
      },
      "list" => {
        todo!();
      },
      type_name => {
        unreachable!("\
          Semantic validation of the AST should have proven \
          that the type '{type_name}' is builtin\
        ");
      }
    }
  }
}
