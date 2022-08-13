use crate::lexer::{Token, TokenStream};
use crate::{ast, ast::Node};

//#region Grammar
peg::parser!{
  pub grammar unit_parser<'source>() for TokenStream<'source> {
    //#region Unit
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
    //#endregion

    //#region Statements
    rule statement() -> Node<ast::Statement>
      = statement_import()
      / statement_effect()
      / statement_class()
      / statement_function()

    //#region Import
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
    //#endregion

    //#region Effect Declaration
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
    //#endregion

    //#region Class Declaration
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
    //#endregion

    //#region Function Declaration
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
    //#endregion

    //#endregion

    //#region Parameters
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
    //#endregion

    //#region Proposition
    rule proposition() -> Node<ast::statement::Proposition>
      = proposition_evaluation()
      / proposition_constraint()

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
    //#endregion

    //#region TypeRef expressions
    rule type_refs() -> Vec<Node<ast::types::TypeRef>>
      = type_refs:(type_ref() ++ [Token::Comma]) [Token::Comma]?
      { type_refs }

    //#region Precedence Climbing
    #[cache_left_rec]
    rule type_ref() -> Node<ast::types::TypeRef>
      = type_ref_expr1()

    #[cache_left_rec]
    rule type_ref_expr1() -> Node<ast::types::TypeRef>
      = l:position!()
        lhs:type_ref_expr1() [Token::OperatorBinOr] rhs:type_ref_expr2()
        r:position!()
        {
          Node::new(
            (l, r),
            ast::types::TypeRef::one_of(vec![lhs, rhs])
          )
        }

    #[cache_left_rec]
    rule type_ref_expr2() -> Node<ast::types::TypeRef>
      = l:position!()
        lhs:type_ref_expr2() [Token::OperatorBinAnd] rhs:type_ref_expr3()
        r:position!()
        {
          Node::new(
            (l, r),
            ast::types::TypeRef::all_of(vec![lhs, rhs])
          )
        }

    #[cache_left_rec]
    rule type_ref_expr3() -> Node<ast::types::TypeRef>
      = l:position!()
        [Token::Negation] t:type_ref_expr4()
        r:position!()
        {
          Node::new(
            (l, r),
            ast::types::TypeRef::not(t)
          )
        }

    #[cache_left_rec]
    rule type_ref_expr4() -> Node<ast::types::TypeRef>
      = type_ref_term()
      / type_ref_recursion()

    rule type_ref_recursion() -> Node<ast::types::TypeRef>
      = [Token::ParenthesisBegin] t:type_ref() [Token::ParenthesisEnd] { t }
    //#endregion

    //#region Terms
    rule type_ref_term() -> Node<ast::types::TypeRef>
      = type_ref_term_generic_symbol()
      / type_ref_term_concrete_symbol()
      / type_ref_term_struct_def()
      / type_ref_term_tuple_def()
      / type_ref_term_function_signature()
      / type_ref_term_literal()

    rule type_ref_term_generic_symbol() -> Node<ast::types::TypeRef>
      = l:position!()
        type_name:symbol_path() type_params:type_params_specialization()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::types::TypeRef::type_name(type_name, type_params)
        )
      }

    rule type_ref_term_concrete_symbol() -> Node<ast::types::TypeRef>
      = l:position!() type_name:symbol_path() r:position!()
      {
        Node::new(
          (l, r),
          ast::types::TypeRef::type_name(type_name, vec![])
        )
      }

    rule type_ref_term_struct_def() -> Node<ast::types::TypeRef>
      = l:position!() struct_def:structure_definition() r:position!()
      {
        Node::new(
          (l, r),
          ast::types::TypeRef::struct_definition(struct_def)
        )
      }

    rule type_ref_term_tuple_def() -> Node<ast::types::TypeRef>
      = l:position!() members:tuple_definition() r:position!()
      {
        Node::new(
          (l, r),
          ast::types::TypeRef::tuple_definition(members)
        )
      }

    rule type_ref_term_function_signature() -> Node<ast::types::TypeRef>
      = l:position!()
        [Token::KeywordFunction]
        [Token::BracketBegin]
          [Token::ParenthesisBegin] params:type_refs() [Token::ParenthesisEnd]
          [Token::Arrow]
          return_type:type_ref()
        [Token::BracketEnd]
        r:position!()
      {
        Node::new(
          (l, r),
          ast::types::TypeRef::function_signature(params, return_type)
        )
      }

    rule type_ref_term_literal() -> Node<ast::types::TypeRef>
      = l:position!() value:literal() r:position!()
      {
        Node::new(
          (l, r),
          ast::types::TypeRef::value(value)
        )
      }
    //#endregion

    //#region Structure Definition
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
    //#endregion

    //#region Tuple Definition
    rule tuple_definition() -> Vec<Node<ast::types::TypeRef>>
      = [Token::ParenthesisBegin] members:type_refs() [Token::ParenthesisEnd]
      { members }
    //#endregion

    //#endregion

    //#region Expressions

    rule expressions() -> Vec<Node<ast::expression::Expression>>
      = exprs:(expression() ** [Token::Comma]) [Token::Comma]?
      { exprs }

    //#region Precedence Climbing
    rule expression() -> Node<ast::expression::Expression>
      = expression_1()

    #[cache_left_rec]
    rule expression_1() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:pattern() [Token::OperatorAssign] rhs:expression_1()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::pattern_match(lhs, rhs)
        )
      }
      / expression_2()

    #[cache_left_rec]
    rule expression_2() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_2() [Token::OperatorIs] rhs:type_ref()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::type_check(lhs, rhs)
        )
      }
      / l:position!()
        lhs:expression_2() [Token::OperatorIs] [Token::OperatorLogicalNot] rhs:type_ref()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::unary_op("not", Node::new(
            (l, r),
            ast::expression::Expression::type_check(lhs, rhs)
          ))
        )
      }
      / expression_3()

    #[cache_left_rec]
    rule expression_3() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_3() [Token::OperatorPipeline] rhs:expression_4()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("|>", lhs, rhs)
        )
      }

    #[cache_left_rec]
    rule expression_4() -> Node<ast::expression::Expression>
      = l:position!()
        [Token::KeywordThrow] e:expression_4()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::unary_op("throw", e)
        )
      }
      / expression_5()

    #[cache_left_rec]
    rule expression_5() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_5() [Token::OperatorLogicalOr] rhs:expression_6()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("or", lhs, rhs)
        )
      }

    #[cache_left_rec]
    rule expression_6() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_6() [Token::OperatorLogicalAnd] rhs:expression_7()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("and", lhs, rhs)
        )
      }

    #[cache_left_rec]
    rule expression_7() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_7() [Token::OperatorBinOr] rhs:expression_8()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("|", lhs, rhs)
        )
      }

    #[cache_left_rec]
    rule expression_8() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_8() [Token::OperatorBinXor] rhs:expression_9()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("^", lhs, rhs)
        )
      }

    #[cache_left_rec]
    rule expression_9() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_9() [Token::OperatorBinAnd] rhs:expression_10()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("&", lhs, rhs)
        )
      }

    #[cache_left_rec]
    rule expression_10() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_10() [Token::OperatorIn] rhs:expression_11()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("in", lhs, rhs)
        )
      }
      / l:position!()
        lhs:expression_10() [Token::OperatorLogicalNot] [Token::OperatorIn] rhs:expression_11()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::unary_op("not", Node::new(
            (l, r),
            ast::expression::Expression::binary_op("in", lhs, rhs)
          ))
        )
      }

    #[cache_left_rec]
    rule expression_11() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_11() [Token::OperatorCmpEQ] rhs:expression_12()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("=", lhs, rhs)
        )
      }
      / l:position!()
        lhs:expression_11() [Token::OperatorCmpNE] rhs:expression_12()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("!=", lhs, rhs)
        )
      }

    #[cache_left_rec]
    rule expression_12() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_12() [Token::OperatorCmpLT] rhs:expression_13()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("<", lhs, rhs)
        )
      }
      / l:position!()
        lhs:expression_12() [Token::OperatorCmpLTE] rhs:expression_13()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("<=", lhs, rhs)
        )
      }
      / l:position!()
        lhs:expression_12() [Token::OperatorCmpGTE] rhs:expression_13()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op(">=", lhs, rhs)
        )
      }
      / l:position!()
        lhs:expression_12() [Token::OperatorCmpGT] rhs:expression_13()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op(">", lhs, rhs)
        )
      }

    #[cache_left_rec]
    rule expression_13() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_13() [Token::OperatorBinLShift] rhs:expression_14()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("<<", lhs, rhs)
        )
      }
      / l:position!()
        lhs:expression_13() [Token::OperatorBinRShift] rhs:expression_14()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op(">>", lhs, rhs)
        )
      }

    #[cache_left_rec]
    rule expression_14() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_14() [Token::OperatorMathAdd] rhs:expression_15()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("+", lhs, rhs)
        )
      }
      / l:position!()
        lhs:expression_14() [Token::OperatorMathSub] rhs:expression_15()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("-", lhs, rhs)
        )
      }

    #[cache_left_rec]
    rule expression_15() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_15() [Token::OperatorMathMul] rhs:expression_16()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("*", lhs, rhs)
        )
      }
      / l:position!()
        lhs:expression_15() [Token::OperatorMathDiv] rhs:expression_16()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("/", lhs, rhs)
        )
      }
      / l:position!()
        lhs:expression_15() [Token::OperatorMathMod] rhs:expression_16()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("%", lhs, rhs)
        )
      }

    #[cache_left_rec]
    rule expression_16() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_16() [Token::OperatorMathPow] rhs:expression_17()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::binary_op("**", lhs, rhs)
        )
      }

    #[cache_left_rec]
    rule expression_17() -> Node<ast::expression::Expression>
      = l:position!()
        [Token::OperatorMathSub] e:expression_term()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::unary_op("-", e)
        )
      }
      / l:position!()
        [Token::OperatorLogicalNot] e:expression_term()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::unary_op("not", e)
        )
      }
      / l:position!()
        [Token::OperatorBinNot] e:expression_term()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::unary_op("~", e)
        )
      }
      / expression_18()

    #[cache_left_rec]
    rule expression_18() -> Node<ast::expression::Expression>
      = l:position!()
        lhs:expression_18() [Token::OperatorAccess] rhs:identifier()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::member_access(lhs, rhs)
        )
      }
      / l:position!()
        func:expression_18()
        [Token::ParenthesisBegin] params:expressions() [Token::ParenthesisEnd]
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::function_call(func, params)
        )
      }
      / l:position!()
        symbol:expression_18() type_params:type_params_specialization()
        r:position!()
      {
        Node::new(
          (l, r),
          ast::expression::Expression::generic_resolve(symbol, type_params)
        )
      }
      / expression_19()

    #[cache_left_rec]
    rule expression_19() -> Node<ast::expression::Expression>
      = expression_term()
      / expression_recursion()

    rule expression_recursion() -> Node<ast::expression::Expression>
      = [Token::ParenthesisBegin] e:expression() [Token::ParenthesisEnd] { e }
    //#endregion

    //#region Terms
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

    //#region Data
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
    //#endregion

    //#region Effect
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
    //#endregion

    //#region Spawn
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
    //#endregion

    //#region Loop
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
    //#endregion

    //#region Match
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
    //#endregion

    //#region Conditional
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
    //#endregion

    //#region Receive
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

    //#endregion

    //#endregion

    //#endregion

    //#region Pattern Matching
    rule pattern() -> Node<ast::expression::Pattern>
      = pattern_symbol()
      / pattern_literal()
      / pattern_tuple()
      / pattern_struct()
      / pattern_list()
      / pattern_list_head_tail()

    //#region Variants
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
    //#endregion

    //#region Tuple Match
    rule tuple_match() -> Vec<Node<ast::expression::Pattern>>
      = [Token::ParenthesisBegin]
        members:(pattern() ** [Token::Comma]) [Token::Comma]?
        [Token::ParenthesisEnd]
      { members }
    //#endregion

    //#region Structure Match
    rule struct_match() -> Vec<(String, Node<ast::expression::Pattern>)>
      = [Token::CurlyBracketBegin]
        members:(struct_member_match() ** [Token::Comma]) [Token::Comma]?
        [Token::CurlyBracketEnd]
      { members }

    rule struct_member_match() -> (String, Node<ast::expression::Pattern>)
      = name:identifier() [Token::Colon] pattern:pattern()
      { (name, pattern) }
    //#endregion

    //#region List Match
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
    //#endregion

    //#endregion

    //#region Structure
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
    //#endregion

    //#region Tuple
    rule tuple() -> Vec<Node<ast::expression::Expression>>
      = [Token::ParenthesisBegin] members:expressions() [Token::ParenthesisEnd]
      { members }
    //#endregion

    //#region List
    rule list() -> Vec<Node<ast::expression::Expression>>
      = [Token::BracketBegin] items:expressions() [Token::BracketEnd]
      { items }
    //#endregion

    //#region Literal
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
    //#endregion

    //#region Misc
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

    //#endregion
  }
}
//#endregion
