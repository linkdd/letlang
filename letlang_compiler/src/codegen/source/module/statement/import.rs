pub use super::Generator;

use letlang_ast::statement::*;


impl<'compiler> Generator<'compiler> {
  pub fn gen_statement_import(&self, data: &ImportStatement) -> String {
    match data.alias.as_ref() {
      None => {
        format!("pub(crate) use lldep_{};", data.path.join("_"))
      },
      Some(alias) => {
        format!("pub(crate) use lldep_{} as lldep_{}", data.path.join("_"), alias)
      }
    }
  }
}

