pub mod result;
pub mod tokens;
pub mod lexer;

lalrpop_mod!(pub grammar, "/parser/grammar.rs");

use crate::{
  ast,
  ast::Node,
  parser::{
    lexer::Lexer,
    grammar::UnitParser,
  },
};

pub struct Parser {
  pub filename: String,
}

impl Parser {
  pub fn new(filename: &String) -> Self {
    Parser {
      filename: filename.clone(),
    }
  }

  pub fn parse(&mut self) -> Result<Node<ast::Unit>, Box<dyn std::error::Error>> {
    let source_code = std::fs::read_to_string(&self.filename)?;

    let lexer = Lexer::new(&self.filename, &source_code[..]);
    let parser = UnitParser::new();
    let unit = parser.parse(self, lexer)?;

    Ok(unit)
  }
}
