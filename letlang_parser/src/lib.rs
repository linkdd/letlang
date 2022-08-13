extern crate logos;

pub mod prelude;
pub mod ast;

mod lexer;
mod grammar;

use ast::Node;


pub fn parse_string(filename: &str, input: &str) -> Result<Node<ast::Unit>, Box<dyn std::error::Error>> {
  let stream = lexer::TokenStream::new(filename, input);
  let ast = grammar::unit_parser::unit(&stream)?;
  Ok(ast)
}


pub fn parse_file(filename: &str) -> Result<Node<ast::Unit>, Box<dyn std::error::Error>> {
  let input = std::fs::read_to_string(filename)?;
  parse_string(filename, &input)
}
