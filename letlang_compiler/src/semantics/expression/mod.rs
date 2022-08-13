use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::{
  Node,
  expression::Expression,
};


impl<V: Visitor> Model<V> {
  pub fn visit_expression(&mut self, node: &Node<Expression>) -> CompilationResult<()> {
    let result = match node.data.as_ref() {
      Expression::Symbol(sym) => {
        self.visit_symbol(sym)
      },
      Expression::Literal(val) => {
        self.visit_literal(val)
      },
      Expression::Structure(structure) => {
        self.visit_structure(structure)
      },
      Expression::Tuple(tuple) => {
        self.visit_tuple(tuple)
      },
      Expression::List(list) => {
        self.visit_list(list)
      },
      Expression::EffectCall(call) => {
        self.visit_effect_call(call)
      },
      Expression::FunctionCall(call) => {
        self.visit_function_call(call)
      },
      Expression::SpawnCall(call) => {
        self.visit_spawn_call(call)
      },
      Expression::GenericResolve(op) => {
        self.visit_generic_resolve(op)
      },
      Expression::MemberAccess(op) => {
        self.visit_member_access(op)
      },
      Expression::TypeCheck(op) => {
        self.visit_typecheck(op)
      },
      Expression::UnaryOperation(op) => {
        self.visit_unary_operation(op)
      },
      Expression::BinaryOperation(op) => {
        self.visit_binary_operation(op)
      },
      Expression::PatternMatch(pattern_match) => {
        self.visit_pattern_match(pattern_match)
      },
      Expression::Loop(controlflow) => {
        self.visit_flow_loop(controlflow)
      },
      Expression::Break(controlflow) => {
        self.visit_flow_break(controlflow)
      },
      Expression::FlowMatch(controlflow) => {
        self.visit_flow_match(controlflow)
      },
      Expression::FlowConditional(controlflow) => {
        self.visit_flow_conditional(controlflow)
      },
      Expression::Receive(block) => {
        self.visit_receive(block)
      },
    };

    Self::locate_error(result, &node.location)
  }
}

mod symbol;
mod literal;
mod containers;
mod calls;
mod operators;
mod controlflow;
mod messaging;
mod pattern;
