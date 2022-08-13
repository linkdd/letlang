extern crate logos;

pub mod prelude;
pub mod ast;

mod lexer;
mod grammar;

use ast::Node;
use std::path::Path;


pub fn parse_string<P: AsRef<Path>>(
  filename: P,
  input: &str
) -> Result<Node<ast::Unit>, Box<dyn std::error::Error>> {
  let stream = lexer::TokenStream::new(
    filename.as_ref().to_path_buf(),
    input,
  )?;
  let ast = grammar::unit_parser::unit(&stream)?;
  Ok(ast)
}


pub fn parse_file<P: AsRef<Path>>(
  filename: P
) -> Result<Node<ast::Unit>, Box<dyn std::error::Error>> {
  let input = std::fs::read_to_string(&filename)?;
  parse_string(&filename, &input)
}
