use string_interner::{
  StringInterner,
  backend::StringBackend,
  symbol::SymbolUsize,
};

pub type AtomInterner = StringInterner<StringBackend<SymbolUsize>>;

pub fn new() -> AtomInterner {
  StringInterner::new()
}
