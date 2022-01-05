use crate::ast::PropositionNode;

#[derive(Debug, Clone)]
pub struct Predicate {
  pub propositions: Vec<PropositionNode>,
}

pub type PredicateNode = Box<Predicate>;
