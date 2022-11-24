use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "expression/flow_match.rs.j2", escape = "none")]
struct FlowMatchTemplate {
  expression: String,
  cases: Vec<FlowMatchCaseTemplate>,
}

struct FlowMatchCaseTemplate {
  pattern: String,
  branch: Vec<String>,
}


impl<'compiler> Generator<'compiler> {
  pub fn gen_flow_match(
    &self,
    location: &LocationInfo,
    data: &FlowMatch,
  ) -> CompilationResult<String> {
    let expression = self.gen_expression(&data.expr)?;

    let mut cases = vec![];

    for (pattern_node, branch_nodes) in data.cases.iter() {
      let pattern = self.gen_pattern(pattern_node)?;
      let mut branch = vec![];

      for node in branch_nodes.iter() {
        branch.push(self.gen_proposition(node)?);
      }

      cases.push(FlowMatchCaseTemplate {
        pattern,
        branch,
      });
    }

    let context = FlowMatchTemplate {
      expression,
      cases,
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate match expression source: {}", e),
      )
    })?;

    Ok(source_code)
  }
}
