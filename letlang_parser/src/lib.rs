#[macro_use] extern crate lalrpop_util;
extern crate logos;

pub mod ast;
pub mod parser;

use pyo3::{prelude::*, exceptions::PyRuntimeError};

#[pyfunction]
fn parse(filename: String) -> PyResult<String> {
  let mut parser = parser::Parser::new(&filename);
  let unit = parser.parse()
    .map_err(|err| {
      PyRuntimeError::new_err(err.to_string())
    })?;

  let output = serde_json::to_string(&unit)
    .map_err(|err| {
      PyRuntimeError::new_err(err.to_string())
    })?;

  Ok(output)
}

#[pymodule]
fn letlang_parser(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(parse, m)?)?;
  Ok(())
}
