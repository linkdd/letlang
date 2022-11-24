use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "expression/flow_cond.rs.j2", escape = "none")]
struct FlowCondTemplate {
  cases: Vec<FlowCondCaseTemplate>,
  else_case: Vec<String>,
}

struct FlowCondCaseTemplate {
  cond: String,
  branch: Vec<String>,
}


impl<'compiler> Generator<'compiler> {
  pub fn gen_flow_cond(
    &self,
    location: &LocationInfo,
    data: &FlowConditional,
  ) -> CompilationResult<String> {
    let mut cases = vec![];

    for (cond_node, branch_nodes) in data.cases.iter() {
      let cond = self.gen_expression(cond_node)?;
      let mut branch = vec![];

      for node in branch_nodes.iter() {
        branch.push(self.gen_proposition(node)?);
      }

      cases.push(FlowCondCaseTemplate {
        cond,
        branch,
      });
    }

    let mut else_case = vec![];

    for node in data.else_case.iter() {
      else_case.push(self.gen_proposition(node)?);
    }

    let context = FlowCondTemplate {
      cases,
      else_case,
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate cond expression source: {}", e),
      )
    })?;

    Ok(source_code)
  }
}
