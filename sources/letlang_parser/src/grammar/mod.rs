use crate::lexer::{Token, TokenStream};
use crate::{ast, ast::Node};

/* #region Grammar */
peg::parser!{
  pub grammar unit_parser<'source>() for TokenStream<'source> {
    /* #region Unit */
    pub rule unit() -> Node<ast::Unit>
      = l:position!() module:module() stmts:statement()* r:position!()
      {
        Node::new(
          (l, r),
          ast::Unit::new(module, stmts)
        )
      }

    rule module() -> Vec<String>
      = [Token::KeywordModule] sym:symbol_path() [Token::Semicolon]
      {
        sym
      }
    /* #endregion */

    /* #region Statements */
    rule statement() -> Node<ast::Statement>
      = statement_import()
      / statement_effect()
      / statement_class()
      / statement_function()

    /* #region Import */
    rule statement_import() -> Node<ast::Statement>
      = l:position!()
          [Token::KeywordImport]
          path:symbol_path()
          alias:statement_import_alias()?
          [Token::Semicolon]
        r:position!()
      {
        Node::new(
          (l, r),
          ast::Statement::import(path, alias)
        )
      }

    rule statement_import_alias() -> String
      = [Token::KeywordAlias] [Token::Identifier(alias)] { alias.clone() }
    /* #endregion */

    /* #region Effect Declaration */
    rule statement_effect() -> Node<ast::Statement>
      = l:position!()
          public:[Token::KeywordPublic]?
          [Token::KeywordEffect] name:identifier()
          [Token::ParenthesisBegin] params:call_params() [Token::ParenthesisEnd]
          [Token::Arrow] return_type:type_ref()
          [Token::Semicolon]
        r:position!()
      {
        Node::new(
          (l, r),
          ast::Statement::effect(
            public.is_some(),
            name,
            params,
            return_type,
          )
        )
      }
    /* #endregion */

    /* #region Class Declaration */
    rule statement_class() -> Node<ast::Statement>
      = statement_class_simple()
      / statement_class_with_constraints()

      rule statement_class_simple() -> Node<ast::Statement>
      = l:position!()
          public:[Token::KeywordPublic]?
          [Token::KeywordClass] name:identifier()
          type_params:type_params_template()?
          [Token::ParenthesisBegin] cons_param:cons_param() [Token::ParenthesisEnd]
          [Token::Semicolon]
        r:position!()
      {
        Node::new(
          (l, r),
          ast::Statement::class(
            public.is_some(),
            name,
            type_params.unwrap_or(vec![]),
            cons_param,
            vec![]
          )
        )
      }

    rule statement_class_with_constraints() -> Node<ast::Statement>
      = l:position!()
          public:[Token::KeywordPublic]?
          [Token::KeywordClass] name:identifier()
          type_params:type_params_template()?
          [Token::ParenthesisBegin] cons_param:cons_param() [Token::ParenthesisEnd]
          [Token::CurlyBracketBegin] body:proposition()+ [Token::CurlyBracketEnd]
        r:position!()
      {
        Node::new(
          (l, r),
          ast::Statement::class(
            public.is_some(),
            name,
            type_params.unwrap_or(vec![]),
            cons_param,
            body
          )
        )
      }
    /* #endregion */

    /* #region Function Declaration */
    rule statement_function() -> Node<ast::Statement>
      = l:position!()
          public:[Token::KeywordPublic]?
          [Token::KeywordFunction] name:identifier()
          type_params:type_params_template()?
          [Token::ParenthesisBegin] call_params:call_params() [Token::ParenthesisEnd]
          [Token::Arrow] return_type:type_ref()
          [Token::CurlyBracketBegin] body:proposition()+ [Token::CurlyBracketEnd]
        r:position!()
      {
        Node::new(
          (l, r),
          ast::Statement::function(
            public.is_some(),
            name,
            type_params.unwrap_or(vec![]),
            call_params,
            return_type,
            body
          )
        )
      }
    /* #endregion */

    /* #endregion */

    /* #region Parameters */
    rule type_params() -> Vec<Node<ast::params::TypeParam>>
      = params:(type_param() ** [Token::Comma]) [Token::Comma]?
      { params }

    rule type_param() -> Node<ast::params::TypeParam>
      = l:position!() id:identifier() r:position!()
      {
        Node::new(
          (l, r),
          ast::params::TypeParam::new(id)
        )
      }

    rule cons_param() -> Node<ast::params::ConsParam>
      = l:position!() param_name:identifier() [Token::Colon] param_type:type_ref() r:position!()
      {
        Node::new(
          (l, r),
          ast::params::ConsParam::new(param_name, param_type)
        )
      }

    rule call_params() -> Vec<Node<ast::params::CallParam>>
      = params:(call_param() ** [Token::Comma]) [Token::Comma]?
      { params }

    rule call_param() -> Node<ast::params::CallParam>
      = l:position!()
        name:identifier()
        [Token::Colon]
        type_ref:type_ref()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::params::CallParam::new(name, type_ref)
        )
      }
    /* #endregion */

    /* #region Proposition */
    rule proposition() -> Node<ast::statement::Proposition>
      = proposition_constraint()
      / proposition_evaluation()

    rule proposition_evaluation() -> Node<ast::statement::Proposition>
      = l:position!() expr:expression() [Token::Semicolon] r:position!()
      {
        Node::new(
          (l, r),
          ast::statement::Proposition::evaluation(expr)
        )
      }

    rule proposition_constraint() -> Node<ast::statement::Proposition>
      = l:position!()
        [Token::KeywordLet]
        symbol_name:identifier()
        [Token::Colon]
        symbol_type:type_ref()
        [Token::Semicolon]
        r:position!()
      {
        Node::new(
          (l, r),
          ast::statement::Proposition::constraint(symbol_name, symbol_type, vec![])
        )
      }
      / l:position!()
        [Token::KeywordLet]
        symbol_name:identifier()
        [Token::Colon]
        symbol_type:type_ref()
        [Token::CurlyBracketBegin]
        checks:proposition_constraint_checks()
        [Token::CurlyBracketEnd]
        [Token::Semicolon]
        r:position!()
      {
        Node::new(
          (l, r),
          ast::statement::Proposition::constraint(symbol_name, symbol_type, checks)
        )
      }

    rule proposition_constraint_checks() -> Vec<Node<ast::expression::Expression>>
      = exprs:(expression() ++ [Token::Comma]) [Token::Comma]?
      { exprs }
    /* #endregion */

    /* #region TypeRef expressions */
    rule type_refs() -> Vec<Node<ast::types::TypeRef>>
      = type_refs:(type_ref() ++ [Token::Comma]) [Token::Comma]?
      { type_refs }

    /* #region Precedence Climbing */
    rule type_ref() -> Node<ast::types::TypeRef> = precedence!{
      l:position!() data:@ r:position!() {
        Node::new((l, r), data)
      }
      --
      lhs:(@) [Token::OperatorBinOr] rhs:@ {
        ast::types::TypeRef::one_of(vec![lhs, rhs])
      }
      --
      lhs:(@) [Token::OperatorBinAnd] rhs:@ {
        ast::types::TypeRef::all_of(vec![lhs, rhs])
      }
      --
      [Token::Negation] t:(@) {
        ast::types::TypeRef::not(t)
      }
      --
      t:type_ref_term() { t }
      [Token::ParenthesisBegin] t:type_ref() [Token::ParenthesisEnd] { t.data }
    }
    /* #endregion */

    /* #region Terms */
    rule type_ref_term() -> Box<ast::types::TypeRef>
      = type_ref_term_generic_symbol()
      / type_ref_term_concrete_symbol()
      / type_ref_term_struct_def()
      / type_ref_term_tuple_def()
      / type_ref_term_function_signature()
      / type_ref_term_literal()

    rule type_ref_term_generic_symbol() -> Box<ast::types::TypeRef>
      = type_name:symbol_path() type_params:type_params_specialization()
      { ast::types::TypeRef::type_name(type_name, type_params) }

    rule type_ref_term_concrete_symbol() -> Box<ast::types::TypeRef>
      = type_name:symbol_path()
      { ast::types::TypeRef::type_name(type_name, vec![]) }

    rule type_ref_term_struct_def() -> Box<ast::types::TypeRef>
      = struct_def:structure_definition()
      { ast::types::TypeRef::struct_definition(struct_def) }

    rule type_ref_term_tuple_def() -> Box<ast::types::TypeRef>
      = members:tuple_definition()
      { ast::types::TypeRef::tuple_definition(members) }

    rule type_ref_term_function_signature() -> Box<ast::types::TypeRef>
      = [Token::KeywordFunction]
        [Token::BracketBegin]
          [Token::ParenthesisBegin] params:type_refs() [Token::ParenthesisEnd]
          [Token::Arrow]
          return_type:type_ref()
        [Token::BracketEnd]
      { ast::types::TypeRef::function_signature(params, return_type) }

    rule type_ref_term_literal() -> Box<ast::types::TypeRef>
      = value:literal()
      { ast::types::TypeRef::value(value) }
    /* #endregion */

    /* #region Structure Definition */
    rule structure_definition() -> Vec<(String, Node<ast::types::TypeRef>)>
      = [Token::CurlyBracketBegin]
        members:struct_definition_members()
        [Token::CurlyBracketEnd]
      { members }

    rule struct_definition_members() -> Vec<(String, Node<ast::types::TypeRef>)>
      = members:(struct_definition_member() ** [Token::Comma]) [Token::Comma]?
      { members }

    rule struct_definition_member() -> (String, Node<ast::types::TypeRef>)
      = name:identifier() [Token::Colon] type_ref:type_ref()
      { (name, type_ref) }
    /* #endregion */

    /* #region Tuple Definition */
    rule tuple_definition() -> Vec<Node<ast::types::TypeRef>>
      = [Token::ParenthesisBegin] members:type_refs() [Token::ParenthesisEnd]
      { members }
    /* #endregion */

    /* #endregion */

    /* #region Expressions */

    rule expressions() -> Vec<Node<ast::expression::Expression>>
      = exprs:(expression() ** [Token::Comma]) [Token::Comma]?
      { exprs }

    /* #region Precedence Climbing */
    rule expression() -> Node<ast::expression::Expression> = precedence!{
      l:position!() data:@ r:position!() {
        Node::new((l, r), data)
      }
      --
      lhs:pattern() [Token::OperatorAssign] rhs:(@) {
        ast::expression::Expression::pattern_match(lhs, rhs)
      }
      --
      lhs:(@) [Token::OperatorIs] rhs:type_ref() {
        ast::expression::Expression::type_check(lhs, rhs, false)
      }
      lhs:(@) [Token::OperatorIs] [Token::OperatorLogicalNot] rhs:type_ref() {
        ast::expression::Expression::type_check(lhs, rhs, true)
      }
      --
      lhs:(@) [Token::OperatorPipeline] rhs:@ {
        ast::expression::Expression::binary_op("|>", lhs, rhs)
      }
      --
      [Token::KeywordThrow] e:(@) {
        ast::expression::Expression::unary_op("throw", e)
      }
      --
      lhs:(@) [Token::OperatorLogicalOr] rhs:@ {
        ast::expression::Expression::binary_op("or", lhs, rhs)
      }
      --
      lhs:(@) [Token::OperatorLogicalAnd] rhs:@ {
        ast::expression::Expression::binary_op("and", lhs, rhs)
      }
      --
      lhs:(@) [Token::OperatorBinOr] rhs:@ {
        ast::expression::Expression::binary_op("|", lhs, rhs)
      }
      --
      lhs:(@) [Token::OperatorBinXor] rhs:@ {
        ast::expression::Expression::binary_op("^", lhs, rhs)
      }
      --
      lhs:(@) [Token::OperatorBinAnd] rhs:@ {
        ast::expression::Expression::binary_op("&", lhs, rhs)
      }
      --
      lhs:(@) [Token::OperatorIn] rhs:@ {
        ast::expression::Expression::binary_op("in", lhs, rhs)
      }
      lhs:(@) [Token::OperatorLogicalNot] [Token::OperatorIn] rhs:@ {
        ast::expression::Expression::binary_op("nin", lhs, rhs)
      }
      --
      lhs:(@) [Token::OperatorCmpEQ] rhs:@ {
        ast::expression::Expression::binary_op("=", lhs, rhs)
      }
      lhs:(@) [Token::OperatorCmpNE] rhs:@ {
        ast::expression::Expression::binary_op("!=", lhs, rhs)
      }
      --
      lhs:(@) [Token::OperatorCmpLT] rhs:@ {
        ast::expression::Expression::binary_op("<", lhs, rhs)
      }
      lhs:(@) [Token::OperatorCmpLTE] rhs:@ {
        ast::expression::Expression::binary_op("<=", lhs, rhs)
      }
      lhs:(@) [Token::OperatorCmpGTE] rhs:@ {
        ast::expression::Expression::binary_op(">=", lhs, rhs)
      }
      lhs:(@) [Token::OperatorCmpGT] rhs:@ {
        ast::expression::Expression::binary_op(">", lhs, rhs)
      }
      --
      lhs:(@) [Token::OperatorBinLShift] rhs:@ {
        ast::expression::Expression::binary_op("<<", lhs, rhs)
      }
      lhs:(@) [Token::OperatorBinRShift] rhs:@ {
        ast::expression::Expression::binary_op(">>", lhs, rhs)
      }
      --
      lhs:(@) [Token::OperatorMathAdd] rhs:@ {
        ast::expression::Expression::binary_op("+", lhs, rhs)
      }
      lhs:(@) [Token::OperatorMathSub] rhs:@ {
        ast::expression::Expression::binary_op("-", lhs, rhs)
      }
      --
      lhs:(@) [Token::OperatorMathMul] rhs:@ {
        ast::expression::Expression::binary_op("*", lhs, rhs)
      }
      lhs:(@) [Token::OperatorMathDiv] rhs:@ {
        ast::expression::Expression::binary_op("/", lhs, rhs)
      }
      lhs:(@) [Token::OperatorMathMod] rhs:@ {
        ast::expression::Expression::binary_op("%", lhs, rhs)
      }
      --
      lhs:(@) [Token::OperatorMathPow] rhs:@ {
        ast::expression::Expression::binary_op("**", lhs, rhs)
      }
      --
      [Token::OperatorMathSub] e:(@) {
        ast::expression::Expression::unary_op("-", e)
      }
      [Token::OperatorLogicalNot] e:(@) {
        ast::expression::Expression::unary_op("not", e)
      }
      [Token::OperatorBinNot] e:(@) {
        ast::expression::Expression::unary_op("~", e)
      }
      --
      lhs:(@) [Token::OperatorAccess] rhs:identifier() {
        ast::expression::Expression::member_access(lhs, rhs)
      }
      func:(@) [Token::ParenthesisBegin] params:expressions() [Token::ParenthesisEnd] {
        ast::expression::Expression::function_call(func, params)
      }
      symbol:(@) type_params:type_params_specialization() {
        ast::expression::Expression::generic_resolve(symbol, type_params)
      }
      --
      t:expression_term() { t.data }
      [Token::ParenthesisBegin] e:expression() [Token::ParenthesisEnd] { e.data }
    }
    /* #endregion */

    /* #region Terms */
    rule expression_term() -> Node<ast::expression::Expression>
      = expression_term_literal()
      / expression_term_struct()
      / expression_term_tuple()
      / expression_term_list()
      / expression_term_symbol()
      / expression_term_effect_call()
      / expression_term_spawn_call()
      / expression_term_loop()
      / expression_term_break()
      / expression_term_match()
      / expression_term_conditional()
      / expression_term_receive()

    /* #region Data */
    rule expression_term_literal() -> Node<ast::expression::Expression>
      = l:position!() lit:literal() r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::literal(lit)
        )
      }

    rule expression_term_struct() -> Node<ast::expression::Expression>
      = l:position!() members:structure() r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::structure(members)
        )
      }

    rule expression_term_tuple() -> Node<ast::expression::Expression>
      = l:position!() members:tuple() r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::tuple(members)
        )
      }

    rule expression_term_list() -> Node<ast::expression::Expression>
      = l:position!() items:list() r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::list(items)
        )
      }

    rule expression_term_symbol() -> Node<ast::expression::Expression>
      = l:position!() sym:symbol_path() r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::symbol(sym)
        )
      }
    /* #endregion */

    /* #region Effect */
    rule expression_term_effect_call() -> Node<ast::expression::Expression>
      = l:position!()
        [Token::KeywordPerform]
        effect_name:symbol_path()
        [Token::ParenthesisBegin]
        params:expressions()
        [Token::ParenthesisEnd]
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::effect_call(effect_name, params)
        )
      }
    /* #endregion */

    /* #region Spawn */
    rule expression_term_spawn_call() -> Node<ast::expression::Expression>
      = l:position!()
        [Token::KeywordSpawn]
        func:expression()
        [Token::ParenthesisBegin]
        params:expressions()
        [Token::ParenthesisEnd]
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::spawn_call(func, params)
        )
      }
    /* #endregion */

    /* #region Loop */
    rule expression_term_loop() -> Node<ast::expression::Expression>
      = l:position!()
        [Token::KeywordLoop]
        [Token::ParenthesisBegin] label:identifier() [Token::ParenthesisEnd]
        [Token::CurlyBracketBegin] body:proposition()+ [Token::CurlyBracketEnd]
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::loop_block(label, body)
        )
      }

    rule expression_term_break() -> Node<ast::expression::Expression>
      = l:position!()
        [Token::KeywordBreak]
        [Token::ParenthesisBegin] label:identifier() [Token::ParenthesisEnd]
        value:expression()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::loop_break(label, value)
        )
      }
    /* #endregion */

    /* #region Match */
    rule expression_term_match() -> Node<ast::expression::Expression>
      = l:position!()
        [Token::KeywordMatch]
        expr:expression()
        [Token::CurlyBracketBegin]
        cases:(expression_term_match_case() ++ [Token::Comma]) [Token::Comma]?
        [Token::CurlyBracketEnd]
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::flow_match(expr, cases)
        )
      }

    rule expression_term_match_case() -> (Node<ast::expression::Pattern>, Vec<Node<ast::statement::Proposition>>)
      = pattern:pattern() [Token::FatArrow]
        [Token::CurlyBracketBegin]
        body:proposition()+
        [Token::CurlyBracketEnd]
      { (pattern, body) }
      / pattern:pattern() [Token::FatArrow]
        l:position!() expr:expression() r:position!()
      {
        (pattern, vec![
          Node::new(
            (l, r),
            ast::statement::Proposition::evaluation(expr)
          )
        ])
      }
    /* #endregion */

    /* #region Conditional */
    rule expression_term_conditional() -> Node<ast::expression::Expression>
      = l:position!()
        [Token::KeywordCond]
        [Token::CurlyBracketBegin]
        cases:(expression_term_conditional_case() ++ [Token::Comma])
        else_case:expression_term_conditional_else_case() [Token::Comma]?
        [Token::CurlyBracketEnd]
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::flow_conditional(cases, else_case)
        )
      }

    rule expression_term_conditional_case() -> (Node<ast::expression::Expression>, Vec<Node<ast::statement::Proposition>>)
      = condition:expression() [Token::FatArrow]
        [Token::CurlyBracketBegin]
        body:proposition()+
        [Token::CurlyBracketEnd]
      { (condition, body) }
      / condition:expression() [Token::FatArrow]
        l:position!() expr:expression() r:position!()
      {
        (condition, vec![
          Node::new(
            (l, r),
            ast::statement::Proposition::evaluation(expr)
          )
        ])
      }

    rule expression_term_conditional_else_case() -> Vec<Node<ast::statement::Proposition>>
      = [Token::KeywordElse] [Token::FatArrow]
        [Token::CurlyBracketBegin]
        body:proposition()+
        [Token::CurlyBracketEnd]
      { body }
      / [Token::KeywordElse] [Token::FatArrow]
        l:position!() expr:expression() r:position!()
      {
        vec![
          Node::new(
            (l, r),
            ast::statement::Proposition::evaluation(expr)
          )
        ]
      }
    /* #endregion */

    /* #region Receive */
    rule expression_term_receive() -> Node<ast::expression::Expression>
      = l:position!()
        [Token::KeywordReceive]
        [Token::CurlyBracketBegin]
        cases:(expression_term_receive_case() ++ [Token::Comma]) [Token::Comma]?
        [Token::CurlyBracketEnd]
        after:expression_term_receive_after()?
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::receive(cases, after)
        )
      }

    rule expression_term_receive_case() -> (Node<ast::expression::Pattern>, Vec<Node<ast::statement::Proposition>>)
      = pattern:pattern() [Token::FatArrow]
        [Token::CurlyBracketBegin]
        body:proposition()+
        [Token::CurlyBracketEnd]
      { (pattern, body) }
      / pattern:pattern() [Token::FatArrow]
        l:position!() expr:expression() r:position!()
        {
          (pattern, vec![
            Node::new(
              (l, r),
              ast::statement::Proposition::evaluation(expr)
            )
          ])
        }

    rule expression_term_receive_after() -> (Node<ast::expression::Expression>, Vec<Node<ast::statement::Proposition>>)
      = [Token::KeywordAfter] timeout:expression()
        [Token::CurlyBracketBegin]
        body:proposition()+
        [Token::CurlyBracketEnd]
      { (timeout, body) }

    /* #endregion */

    /* #endregion */

    /* #endregion */

    /* #region Pattern Matching */
    rule pattern() -> Node<ast::expression::Pattern>
      = pattern_symbol()
      / pattern_literal()
      / pattern_tuple()
      / pattern_struct()
      / pattern_list()
      / pattern_list_head_tail()

    /* #region Variants */
    rule pattern_symbol() -> Node<ast::expression::Pattern>
      = l:position!() sym:identifier() r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Pattern::symbol(sym)
        )
      }

    rule pattern_literal() -> Node<ast::expression::Pattern>
      = l:position!() lit:literal() r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Pattern::literal(lit)
        )
      }

    rule pattern_tuple() -> Node<ast::expression::Pattern>
      = l:position!() members:tuple_match() r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Pattern::tuple(members)
        )
      }

    rule pattern_struct() -> Node<ast::expression::Pattern>
      = l:position!() members:struct_match() r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Pattern::structure(members)
        )
      }

    rule pattern_list() -> Node<ast::expression::Pattern>
      = l:position!() items:list_match() r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Pattern::list(items)
        )
      }

    rule pattern_list_head_tail() -> Node<ast::expression::Pattern>
      = l:position!() head_tail:list_head_tail_match() r:position!()
      {
        let (head, tail) = head_tail;
        Node::new(
          (l, r),
          ast::expression::Pattern::list_head_tail(head, tail)
        )
      }
    /* #endregion */

    /* #region Tuple Match */
    rule tuple_match() -> Vec<Node<ast::expression::Pattern>>
      = [Token::ParenthesisBegin]
        members:(pattern() ** [Token::Comma]) [Token::Comma]?
        [Token::ParenthesisEnd]
      { members }
    /* #endregion */

    /* #region Structure Match */
    rule struct_match() -> Vec<(String, Node<ast::expression::Pattern>)>
      = [Token::CurlyBracketBegin]
        members:(struct_member_match() ** [Token::Comma]) [Token::Comma]?
        [Token::CurlyBracketEnd]
      { members }

    rule struct_member_match() -> (String, Node<ast::expression::Pattern>)
      = name:identifier() [Token::Colon] pattern:pattern()
      { (name, pattern) }
    /* #endregion */

    /* #region List Match */
    rule list_match() -> Vec<Node<ast::expression::Pattern>>
      = [Token::BracketBegin]
        items:(pattern() ** [Token::Comma]) [Token::Comma]?
        [Token::BracketEnd]
      { items }

    rule list_head_tail_match() -> (Node<ast::expression::Pattern>, Node<ast::expression::Pattern>)
      = [Token::BracketBegin]
        head:pattern()
        [Token::OperatorBinOr]
        tail:pattern()
        [Token::BracketEnd]
      { (head, tail) }
    /* #endregion */

    /* #endregion */

    /* #region Structure */
    rule structure() -> Vec<(String, Node<ast::expression::Expression>)>
      = [Token::CurlyBracketBegin]
        members:structure_members()
        [Token::CurlyBracketEnd]
      { members }

    rule structure_members() -> Vec<(String, Node<ast::expression::Expression>)>
      = members:(structure_member() ** [Token::Comma]) [Token::Comma]?
      { members }

    rule structure_member() -> (String, Node<ast::expression::Expression>)
      = member_name:identifier() [Token::Colon] member_val:expression()
      { (member_name, member_val) }
    /* #endregion */

    /* #region Tuple */
    rule tuple() -> Vec<Node<ast::expression::Expression>>
      = [Token::ParenthesisBegin] members:expressions() [Token::ParenthesisEnd]
      { members }
    /* #endregion */

    /* #region List */
    rule list() -> Vec<Node<ast::expression::Expression>>
      = [Token::BracketBegin] items:expressions() [Token::BracketEnd]
      { items }
    /* #endregion */

    /* #region Literal */
    rule literal() -> Node<ast::expression::Literal>
      = literal_true()
      / literal_false()
      / literal_int2()
      / literal_int8()
      / literal_int10()
      / literal_int16()
      / literal_float()
      / literal_string()
      / literal_atom()

    rule literal_true() -> Node<ast::expression::Literal>
      = l:position!() [Token::True] r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Literal::boolean(true)
        )
      }

    rule literal_false() -> Node<ast::expression::Literal>
      = l:position!() [Token::False] r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Literal::boolean(false)
        )
      }

    rule literal_int2() -> Node<ast::expression::Literal>
      = l:position!() [Token::IntegerBase2(n)] r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Literal::number(*n)
        )
      }

    rule literal_int8() -> Node<ast::expression::Literal>
      = l:position!() [Token::IntegerBase8(n)] r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Literal::number(*n)
        )
      }

    rule literal_int10() -> Node<ast::expression::Literal>
      = l:position!() [Token::IntegerBase10(n)] r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Literal::number(*n)
        )
      }

    rule literal_int16() -> Node<ast::expression::Literal>
      = l:position!() [Token::IntegerBase16(n)] r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Literal::number(*n)
        )
      }

    rule literal_float() -> Node<ast::expression::Literal>
      = l:position!() [Token::Float(n)] r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Literal::number(*n)
        )
      }

    rule literal_string() -> Node<ast::expression::Literal>
      = l:position!() [Token::String(s)] r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Literal::string(s.clone())
        )
      }

    rule literal_atom() -> Node<ast::expression::Literal>
      = l:position!() [Token::Atom(a)] r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Literal::atom(a.clone())
        )
      }
    /* #endregion */

    /* #region Misc */
    rule type_params_template() -> Vec<Node<ast::params::TypeParam>>
      = [Token::OperatorCmpLT] params:type_params() [Token::OperatorCmpGT]
      { params }

    rule type_params_specialization() -> Vec<Node<ast::types::TypeRef>>
      = [Token::OperatorCmpLT] params:type_refs() [Token::OperatorCmpGT]
      { params }

    rule symbol_path() -> Vec<String>
      = identifier() ++ [Token::DoubleColon]

    rule identifier() -> String
      = [Token::Identifier(id)] { id.clone() }

    /* #endregion */
  }
}
/* #endregion */
