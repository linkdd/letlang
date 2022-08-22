use crate::prelude::*;
use crate::semantics::Visitor;
use letlang_parser::ast::expression::Atom;

use string_interner::{
  StringInterner,
  backend::StringBackend,
  symbol::SymbolUsize,
};

pub type AtomInterner = StringInterner<StringBackend<SymbolUsize>>;

#[derive(Clone)]
pub struct AtomInternerPhase {
  interner: AtomInterner,
}

impl AtomInternerPhase {
  pub fn new() -> Self {
    Self { interner: StringInterner::new() }
  }

  pub fn get_interner(&mut self) -> &mut AtomInterner {
    &mut self.interner
  }
}

impl Visitor for AtomInternerPhase {
  fn match_node(&self, node: &dyn std::any::Any) -> bool {
    match node.downcast_ref::<Atom>() {
      Some(_) => true,
      None => false,
    }
  }

  fn visit_node(&mut self, node: &dyn std::any::Any) -> CompilationResult<()> {
    if let Some(Atom(a)) = node.downcast_ref::<Atom>() {
      self.interner.get_or_intern(a);
    }

    Ok(())
  }
}
