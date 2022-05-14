use string_interner::{
  StringInterner,
  Symbol,
  backend::StringBackend,
  symbol::SymbolUsize
};

use crate::{Value, PrimitiveValue};

pub type AtomSymbol = SymbolUsize;
pub type AtomTable = StringInterner<StringBackend<AtomSymbol>>;

pub struct Context {
  pub atom_table: AtomTable,
}

impl Context {
  pub fn new() -> Self {
    Self { atom_table: AtomTable::new() }
  }

  pub fn get_atom(&mut self, repr: &str) -> usize {
    self.atom_table.get_or_intern(repr).to_usize()
  }

  pub fn get_atom_repr(&self, val: usize) -> String {
    let sym = AtomSymbol::try_from_usize(val)
      .expect("invalid atom symbol");
    let atom = self.atom_table.resolve(sym)
      .expect("unknown atom symbol");

    atom.to_string()
  }

  pub fn format_primitive_value(&self, llpval: &PrimitiveValue) -> String {
    match llpval {
      PrimitiveValue::Boolean(val) => format!("{}", val),
      PrimitiveValue::Number(val) => format!("{}", val),
      PrimitiveValue::String(val) => format!("{}", snailquote::escape(val.as_str())),
      PrimitiveValue::Atom(val) => self.get_atom_repr(*val),
    }
  }

  pub fn format_value(&self, llval: &Value) -> String {
    match llval {
      Value::Primitive(pval) => self.format_primitive_value(pval),
      Value::Tuple(vals) => {
        let vals_str = vals
          .iter()
          .map(|val| self.format_value(val))
          .collect::<Vec<String>>()
          .join(", ");

        format!("({})", vals_str)
      }
    }
  }
}
