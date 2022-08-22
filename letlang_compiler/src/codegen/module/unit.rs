use crate::prelude::*;
use letlang_parser::{ast, ast::Node};
use super::CodeGenerator;

impl<'gen, 'ctx> CodeGenerator<'gen, 'ctx> {
  pub fn gen_unit(&self, node: &Node<ast::Unit>) -> CompilationResult<Vec<String>> {
    let mut symbols = vec![];

    for statement in node.data.statements.iter() {
      let symbol_code = self.gen_statement(statement)?;
      symbols.push(symbol_code);
    }

    Ok(symbols)
  }
}
