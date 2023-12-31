pub mod symbol;
pub mod env;
pub mod stack;

pub use self::{env::EnvRef, symbol::SymbolKind};
use self::stack::EnvironmentStack;

pub struct ScopeBuilder {
  pub(crate) env_stack: EnvironmentStack,
}

impl ScopeBuilder {
  pub fn new(/* bmi */) -> Self {
    let mut env_stack = EnvironmentStack::new();
    env_stack.enter_scope();
    // TODO: add bmi
    Self { env_stack }
  }
}
