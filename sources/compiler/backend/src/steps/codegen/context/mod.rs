use crate::steps::scope::SymbolKind;

pub struct CodeGenerator {
  pub bmi_modpath: Vec<String>,
  pub bmi_syms: Vec<(String, SymbolKind)>,
}

impl CodeGenerator {
  pub fn new() -> Self {
    Self {
      bmi_modpath: vec![],
      bmi_syms: vec![],
    }
  }
}
