use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::{Node, expression::*};


impl<V: Visitor> Model<V> {
  pub fn visit_pattern(&mut self, node: &Node<Pattern>) -> CompilationResult<()> {
    let result = match node.data.as_ref() {
      Pattern::Symbol(sym) => {
        self.visit_symbol(sym)
      },
      Pattern::Literal(lit) => {
        self.visit_literal(lit)
      },
      Pattern::Tuple(pattern) => {
        self.visit_pattern_tuple(pattern)
      },
      Pattern::Struct(pattern) => {
        self.visit_pattern_struct(pattern)
      },
      Pattern::List(pattern) => {
        self.visit_pattern_list(pattern)
      },
      Pattern::ListHeadTail(pattern) => {
        self.visit_pattern_list_head_tail(pattern)
      }
    };

    Self::locate_error(result, &node.location)
  }

  fn visit_pattern_tuple(&mut self, pattern: &TuplePattern) -> CompilationResult<()> {
    self.call_visitor(pattern)?;

    for member_pattern in pattern.members.iter() {
      self.visit_pattern(member_pattern)?;
    }

    Ok(())
  }

  fn visit_pattern_struct(&mut self, pattern: &StructPattern) -> CompilationResult<()> {
    self.call_visitor(pattern)?;

    for (_, member_pattern) in pattern.members.iter() {
      self.visit_pattern(member_pattern)?;
    }

    Ok(())
  }

  fn visit_pattern_list(&mut self, pattern: &ListPattern) -> CompilationResult<()> {
    self.call_visitor(pattern)?;

    for item_pattern in pattern.items.iter() {
      self.visit_pattern(item_pattern)?;
    }

    Ok(())
  }

  fn visit_pattern_list_head_tail(&mut self, pattern: &ListHeadTailPattern) -> CompilationResult<()> {
    self.call_visitor(pattern)?;

    self.visit_pattern(&pattern.head)?;
    self.visit_pattern(&pattern.tail)?;

    Ok(())
  }
}
