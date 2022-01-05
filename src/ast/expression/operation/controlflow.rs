use crate::ast::{
  ExpressionNode,
  StatementNode,
  TypeConstraintNode,
  VariableNameNode,
  FunctionCallNode,
};

#[derive(Debug, Clone)]
pub enum ControlFlowOperation {
  Do {
    block: Vec<StatementNode>,
    effect_clauses: Vec<EffectClauseNode>,
    catch_clauses: Vec<CatchClauseNode>,
    finally_clause: Option<Vec<StatementNode>>,
  },
  If {
    cond: ExpressionNode,
    true_block: Vec<StatementNode>,
    false_block: Option<Vec<StatementNode>>,
  },
  Match {
    val: ExpressionNode,
    cases: Vec<MatchCaseNode>,
  },
  ForLoop {
    iterator: VariableNameNode,
    iterable: ExpressionNode,
    block: Vec<StatementNode>,
  },
}

#[derive(Debug, Clone)]
pub struct EffectClause {
  pub pattern: FunctionCallNode,
  pub block: Vec<StatementNode>,
}

pub type EffectClauseNode = Box<EffectClause>;

#[derive(Debug, Clone)]
pub struct CatchClause {
  pub pattern: ExpressionNode,
  pub block: Vec<StatementNode>,
}

pub type CatchClauseNode = Box<CatchClause>;

#[derive(Debug, Clone)]
pub struct MatchCase {
  pub pattern: TypeConstraintNode,
  pub guard: Option<ExpressionNode>,
  pub block: Vec<StatementNode>,
}

pub type MatchCaseNode = Box<MatchCase>;
