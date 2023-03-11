use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "expression/flow_do.rs.j2", escape = "none")]
struct FlowDoTemplate {
  body: Vec<String>,
  effect_handlers: Vec<FlowDoInterceptTemplate>,
  exception_handlers: Vec<FlowDoCatchTemplate>,
}

struct FlowDoInterceptTemplate {
  pattern: FlowDoEffectPatternTemplate,
  body: Vec<String>,
}

struct FlowDoEffectPatternTemplate {
  name: String,
  params: Vec<String>,
}

struct FlowDoCatchTemplate {
  pattern_code: String,
  body: Vec<String>,
}

impl<'compiler> Generator<'compiler> {
  pub fn gen_flow_do(
    &self,
    location: &LocationInfo,
    data: &FlowDoBlock,
  ) -> CompilationResult<String> {
    let mut body = vec![];

    for proposition in data.body.iter() {
      let code = self.gen_proposition(proposition)?;
      body.push(code);
    }

    let mut effect_handlers = vec![];

    for (effect_pattern, effect_body) in data.effect_handlers.iter() {
      let pattern = self.gen_effect_pattern(effect_pattern)?;

      let mut body = vec![];

      for proposition in effect_body.iter() {
        let code = self.gen_proposition(proposition)?;
        body.push(code);
      }

      effect_handlers.push(FlowDoInterceptTemplate {
        pattern,
        body,
      });
    }

    let mut exception_handlers = vec![];

    for (exception_pattern, exception_body) in data.exception_handlers.iter() {
      let pattern_code = self.gen_pattern(exception_pattern)?;

      let mut body = vec![];

      for proposition in exception_body.iter() {
        let code = self.gen_proposition(proposition)?;
        body.push(code);
      }

      exception_handlers.push(FlowDoCatchTemplate {
        pattern_code,
        body,
      });
    }

    let context = FlowDoTemplate {
      body,
      effect_handlers,
      exception_handlers,
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate do{{}} expression source: {}", e),
      )
    })?;

    Ok(source_code)
  }

  fn gen_effect_pattern(
    &self,
    node: &Node<EffectPattern>,
  ) -> CompilationResult<FlowDoEffectPatternTemplate> {
    let name = match node.data.effect_name.scope() {
      None => format!("format!(\"{{LL_MOD_PATH}}::{{}}\", {:?})", node.data.effect_name.to_string()),
      Some(..) => format!("{:?}", node.data.effect_name.to_string()),
    };

    let mut params = vec![];

    for param in node.data.effect_params.iter() {
      let code = self.gen_pattern(param)?;
      params.push(code);
    }

    Ok(FlowDoEffectPatternTemplate {
      name,
      params,
    })
  }
}
