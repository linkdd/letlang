use crate::prelude::*;
use crate::scope::*;
pub use super::Generator;

use letlang_ast::types::*;


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
      SymbolKind::Class { builtin: true, .. } => {
        Ok(self.gen_builtin_class(data))
      },
      SymbolKind::Class { builtin: false, .. } => {
        self.gen_custom_class(data)
      },
      SymbolKind::TypeParameter { .. } => {
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

  fn gen_custom_class(
    &self,
    data: &TypeName,
  ) -> CompilationResult<String> {
    let mut type_params = vec![];

    for type_param_node in data.type_params.iter() {
      type_params.push(self.gen_typeref(type_param_node)?);
    }

    let class_path = match data.symbol.scope() {
      None => {
        let class_name = data.symbol.name();
        format!("symbol_{class_name}::class_{class_name}")
      },
      Some(module_scope) => {
        let crate_name = format!("lldep_{}", module_scope.replace("::", "_"));
        let class_name = data.symbol.name();
        format!("{crate_name}::symbol_{class_name}::class_{class_name}")
      }
    };

    Ok(format!("{class_path}::new({})", type_params.join(",\n")))
  }
}
