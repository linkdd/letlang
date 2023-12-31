use proc_macro::TokenStream;
use syn::parse_macro_input;

mod parser;
mod codegen;

#[proc_macro]
pub fn model(input: TokenStream) -> TokenStream {
  let model_info = parse_macro_input!(input as parser::model::ModelInfo);
  codegen::model::generate(model_info)
}

#[proc_macro]
pub fn visit(input: TokenStream) -> TokenStream {
  let visit_args = parse_macro_input!(input as parser::visit::VisitArgs);
  codegen::visit::generate(visit_args.first, visit_args.extra)
}

#[proc_macro]
pub fn visit_many(input: TokenStream) -> TokenStream {
  let visit_args = parse_macro_input!(input as parser::visit::VisitArgs);
  codegen::visit_many::generate(visit_args.first, visit_args.extra)
}
