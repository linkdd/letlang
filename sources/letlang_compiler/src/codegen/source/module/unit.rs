use crate::prelude::*;
pub use super::Generator;

use letlang_ast::*;

impl<'compiler> Generator<'compiler> {
  pub fn gen_unit(&self, node: &Node<Unit>) -> CompilationResult<Vec<String>> {
    let mut statements_source = vec![];

    for statement_node in node.data.statements.iter() {
      let statement_source = self.gen_statement(statement_node)?;
      statements_source.push(statement_source);
    }

    Ok(statements_source)
  }
}
