#[derive(Clone)]
pub enum SymbolKind {
  Module { scope_id: usize },
  Effect { call_param_count: usize },
  Class { type_param_count: usize, builtin: bool },
  Function { type_param_count: usize, call_param_count: usize },
  TypeParameter { index: usize },
  ConsParameter,
  CallParameter { index: usize },
  Variable,
}
