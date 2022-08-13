use crate::prelude::*;
use crate::semantics::Visitor;
use letlang_parser::ast::expression::Literal;

use string_interner::{
  StringInterner,
  backend::StringBackend,
  symbol::SymbolUsize,
};

pub struct AtomInternerPhase {
  interner: StringInterner<StringBackend<SymbolUsize>>,
}

impl AtomInternerPhase {
  pub fn new() -> Self {
    Self { interner: StringInterner::new() }
  }
}

impl Visitor for AtomInternerPhase {
  fn match_node(&self, node: &dyn std::any::Any) -> bool {
    if let Some(lit) = node.downcast_ref::<Literal>() {
      match lit {
        Literal::Atom(_) => {
          true
        },
        _ => false,
      }
    }
    else {
      false
    }
  }

  fn visit_node(&mut self, node: &dyn std::any::Any) -> CompilationResult<()> {
    let lit = node.downcast_ref::<Literal>().unwrap();
    if let Literal::Atom(a) = lit {
      self.interner.get_or_intern(a);
    }

    Ok(())
  }
}
