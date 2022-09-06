use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "expression_effect_call.rs.j2", escape = "none")]
struct EffectCallTemplate {
  effect_code: String,
  call_params: Vec<String>,
}



impl<'compiler> Generator<'compiler> {
  pub fn gen_effect_call(
    &self,
    location: &LocationInfo,
    data: &EffectCall,
  ) -> CompilationResult<String> {
    let effect_code = match data.effect_name.scope() {
      None => {
        let effect_name = data.effect_name.name();
        format!("symbol_{effect_name}::effect_{effect_name} {{}}")
      },
      Some(scope_name) => {
        let crate_name = format!("lldep_{}", scope_name.replace("::", "_"));
        let effect_name = data.effect_name.name();
        format!("{crate_name}::symbol_{effect_name}::effect_{effect_name} {{}}")
      }
    };

    let mut call_params = vec![];

    for call_param_node in data.params.iter() {
      let call_param_code = self.gen_expression(call_param_node)?;
      call_params.push(call_param_code);
    }

    let context = EffectCallTemplate {
      effect_code,
      call_params,
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate effect call source: {}", e),
      )
    })?;

    Ok(source_code)
  }
}