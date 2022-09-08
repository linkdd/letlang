use crate::core::pattern::Pattern;
use crate::core::{
  context::TaskContext,
  utils::Locals,
};
use crate::repr::Value;

use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;


pub struct SymbolPattern {
  pub name: &'static str,
}

#[async_trait]
impl Pattern for SymbolPattern {
  async fn match_(
    &self,
    _context: Arc<Mutex<TaskContext>>,
    locals: &mut Locals,
    value: &Value,
  ) -> Result<(), ()> {
    match locals.lookup_symbol(self.name) {
      None => {
        locals.register_symbol(&self.name, value.clone());
      },
      Some(val) => {
        if val != value {
          return Err(());
        }
      }
    }

    Ok(())
  }
}
