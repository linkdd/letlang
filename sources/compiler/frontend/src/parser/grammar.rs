use ast_core::*;
use nonempty::{NonEmpty, nonempty};

use crate::{
  ast::*,
  lexer::{Token, TokenStream},
  parser::sourcemap::SourceMapLookup,
  prelude::SourceLocation,
};

peg::parser!{
  pub grammar unit_parser<'source, 'tokens>(srcmap: &SourceMapLookup<'source, 'tokens>) for TokenStream<'source> {
    pub rule unit() -> AST<SourceLocation<'source>>
      = module:module()
      {
        AST(module)
      }

    /* #region Module statement */
    rule module() -> Node<Module<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        path:("module" p:module_path() [Token::Semicolon] { p })
        imports:imports()
        declarations:declarations()
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Module { path, imports, declarations },
        )
      }

    rule module_path() -> NonEmpty<Node<Identifier, SourceLocation<'source>>>
      = segments:(identifier() ++ [Token::ModSep])
      {
        NonEmpty::from_vec(segments).unwrap()
      }
    /* #endregion */

    /* #region Import statement */
    rule imports() -> Vec<Node<Import<SourceLocation<'source>>, SourceLocation<'source>>>
      = statements:import()*
      {
        statements
      }

    rule import() -> Node<Import<SourceLocation<'source>>, SourceLocation<'source>>
      = import_symbols()
      / import_module()

    rule import_symbols() -> Node<Import<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        "from" path:module_path() "import"
        [Token::BraceBegin]
        symbols:(
          syms:(s:import_symbol() ++ [Token::Comma] [Token::Comma]? { s })
          { NonEmpty::from_vec(syms).unwrap() }
        )
        [Token::BraceEnd]
        [Token::Semicolon]
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Import::Symbol { path, symbols }
        )
      }

    rule import_symbol() -> (Node<Identifier, SourceLocation<'source>>, Option<Node<Identifier, SourceLocation<'source>>>)
      = symbol:identifier() alias:("as" a:identifier() { a })?
      {
        (symbol, alias)
      }

    rule import_module() -> Node<Import<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        "import" path:module_path()
        alias:("as" a:identifier() { a })?
        [Token::Semicolon]
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Import::Module { path, alias }
        )
      }
    /* #endregion */

    /* #region Declaration statement */
    rule declarations() -> Vec<Node<NamedDeclaration<SourceLocation<'source>>, SourceLocation<'source>>>
      = decls:named_declaration()*
      {
        decls
      }

    rule named_declaration() -> Node<NamedDeclaration<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        "let"
        public:(p:"pub"? { p.is_some() })
        name:identifier()
        type_params:(
          t:([Token::LT] i:identifier() ++ [Token::Comma] [Token::Comma]? [Token::GT] { i })?
          { t.unwrap_or(vec![]) }
        )
        [Token::Colon]
        declaration:declaration()
        [Token::Semicolon]
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          NamedDeclaration { public, name, type_params, declaration }
        )
      }

    rule declaration() -> Node<Declaration<SourceLocation<'source>>, SourceLocation<'source>>
      = declaration_class()
      / declaration_effect()
      / declaration_function()

    /* #endregion */

    /* #region Class declaration */
    rule declaration_class() -> Node<Declaration<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        "class" [Token::BracketBegin] cons_param:type_() [Token::BracketEnd]
        clauses:(
          c:(
            [Token::BraceBegin]
            c:clause() ++ [Token::Comma] [Token::Comma]?
            [Token::BraceEnd]
            { c }
          )?
          { c.unwrap_or(vec![]) }
        )
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Declaration::Class(Class { cons_param, clauses })
        )
      }
    /* #endregion */

    /* #region Effect declaration */
    rule declaration_effect() -> Node<Declaration<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        "effect" [Token::BracketBegin]
        [Token::ParenBegin] params:(p:type_() ** [Token::Comma] [Token::Comma]? { p }) [Token::ParenEnd]
        [Token::Arrow]
        return_type:type_()
        [Token::BracketEnd]
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Declaration::Effect(Effect { params, return_type })
        )
      }
    /* #endregion */

    /* #region Function declaration */
    rule declaration_function() -> Node<Declaration<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        tailrec:(("func" { false }) / ("tailrec" { true }))
        [Token::BracketBegin]
        [Token::ParenBegin] params:(p:type_() ** [Token::Comma] [Token::Comma]? { p }) [Token::ParenEnd]
        [Token::Arrow]
        return_type:type_()
        [Token::BracketEnd]
        [Token::BraceBegin]
        clauses:(
          c:(c:clause() ++ [Token::Comma] [Token::Comma]? { c })
          { NonEmpty::from_vec(c).unwrap() }
        )
        [Token::BraceEnd]
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Declaration::Function(Function { tailrec, params, return_type, clauses })
        )
      }
    /* #endregion */

    /* #region Type */
    rule type_() -> Node<Type<SourceLocation<'source>>, SourceLocation<'source>>
      = precedence!{
        l:position!() data:@ r:position!() {
          Node::new(srcmap.token_span(l, r), data)
        }
        --
        lhs:(@) [Token::BitOr] rhs:@ {
          Type::OneOf { lhs, rhs }
        }
        --
        lhs:(@) [Token::BitAnd] rhs:@ {
          Type::AllOf { lhs, rhs }
        }
        --
        [Token::Negation] t:(@) {
          Type::Not(t)
        }
        --
        t:type_term() { t }
        [Token::ParenBegin] t:type_() [Token::ParenEnd] { t.get_data().clone() }
      }

    rule type_term() -> Type<SourceLocation<'source>>
      = type_ref()
      / type_literal()
      / type_tuple()
      / type_namedtuple()

    rule type_ref() -> Type<SourceLocation<'source>>
      = name:symbol_path()
        type_params:(
          t:([Token::LT] t:type_() ++ [Token::Comma] [Token::Comma]? [Token::GT] { t })?
          { t.unwrap_or(vec![]) }
        )
      {
        Type::Reference { name, type_params }
      }

    rule type_literal() -> Type<SourceLocation<'source>>
      = lit:literal()
      {
        Type::Literal(lit)
      }

    rule type_tuple() -> Type<SourceLocation<'source>>
      = items:(
          (
            [Token::ParenBegin]
            item:type_() [Token::Comma]
            [Token::ParenEnd]
            { vec![item] }
          ) / (
            [Token::ParenBegin]
            items:type_() **<2,> [Token::Comma]
            [Token::Comma]?
            [Token::ParenEnd]
            { items }
          ) / (
            [Token::ParenBegin]
            [Token::ParenEnd]
            { vec![] }
          )
        )
      {
        Type::Tuple(items)
      }

    rule type_namedtuple() -> Type<SourceLocation<'source>>
      = items:(
          (
            [Token::BraceBegin]
            items:type_namedtuple_pair() ++ [Token::Comma] [Token::Comma]?
            [Token::BraceEnd]
            { items }
          ) / (
            [Token::BraceBegin]
            [Token::BraceEnd]
            { vec![] }
          )
        )
      {
        Type::NamedTuple(items)
      }

    rule type_namedtuple_pair() -> (Node<Identifier, SourceLocation<'source>>, Node<Type<SourceLocation<'source>>, SourceLocation<'source>>)
      = key:identifier() [Token::Colon] typ:type_()
      {
        (key, typ)
      }

    /* #endregion */

    /* #region Pattern */
    rule clause() -> Node<Clause<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        patterns:(
          (
            [Token::ParenBegin] [Token::ParenEnd]
            { vec![] }
          ) / (
            [Token::ParenBegin]
            patterns:pattern() ++ [Token::Comma] [Token::Comma]?
            [Token::ParenEnd]
            { patterns }
          )
        )
        guard:("when" e:expression() { e })?
        [Token::Arrow]
        body:(
          (
            [Token::BraceBegin]
            e:expression() ++ [Token::Semicolon] [Token::Semicolon]
            [Token::BraceEnd]
            { NonEmpty::from_vec(e).unwrap() }
          ) / (
            e:expression()
            { nonempty![e] }
          )
        )
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Clause { patterns, guard, body }
        )
      }

    rule pattern() -> Node<Pattern<SourceLocation<'source>>, SourceLocation<'source>>
      = pattern_ignore()
      / pattern_binding()
      / pattern_literal()
      / pattern_tuple()
      / pattern_namedtuple()
      / pattern_list()
      / pattern_list_headtail()
      / pattern_eval()

    rule pattern_ignore() -> Node<Pattern<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        [Token::PatternIgnore]
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Pattern::Ignore,
        )
      }

    rule pattern_binding() -> Node<Pattern<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        ident:identifier()
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Pattern::Binding(ident),
        )
      }

    rule pattern_literal() -> Node<Pattern<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        lit:literal()
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Pattern::Literal(lit),
        )
      }

    rule pattern_tuple() -> Node<Pattern<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        t:(
          (
            [Token::ParenBegin]
            strict:(s:[Token::Ellipsis]? [Token::Comma]? { s.is_none() })
            [Token::ParenEnd]
            { (vec![], strict) }
          ) / (
            [Token::ParenBegin]
            items:pattern() ++ [Token::Comma]
            strict:([Token::Comma] s:[Token::Ellipsis]? { s.is_none() })
            [Token::Comma]?
            [Token::ParenEnd]
            { (items, strict) }
          )
        )
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Pattern::Tuple { items: t.0, strict: t.1 },
        )
      }

    rule pattern_namedtuple() -> Node<Pattern<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        s:(
          (
            [Token::BraceBegin]
            strict:(s:[Token::Ellipsis]? [Token::Comma]? { s.is_none() })
            [Token::BraceEnd]
            { (vec![], strict) }
          ) / (
            [Token::BraceBegin]
            items:pattern_namedtuple_pair() ++ [Token::Comma]
            strict:([Token::Comma] s:[Token::Ellipsis]? { s.is_none() })
            [Token::Comma]?
            [Token::BraceEnd]
            { (items, strict) }
          )
        )
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Pattern::NamedTuple { items: s.0, strict: s.1 },
        )
      }

    rule pattern_namedtuple_pair() -> (Node<Identifier, SourceLocation<'source>>, Node<Pattern<SourceLocation<'source>>, SourceLocation<'source>>)
      = key:identifier() [Token::Colon] pattern:pattern()
      { (key, pattern) }

    rule pattern_list() -> Node<Pattern<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        list:(
          (
            [Token::BracketBegin]
            strict:(s:[Token::Ellipsis]? [Token::Comma]? { s.is_none() })
            [Token::BracketEnd]
            { (vec![], strict) }
          ) / (
            [Token::BracketBegin]
            items:pattern() ++ [Token::Comma]
            strict:([Token::Comma] s:[Token::Ellipsis]? { s.is_none() })
            [Token::Comma]?
            [Token::BracketEnd]
            { (items, strict) }
          )
        )
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Pattern::List { items: list.0, strict: list.1 },
        )
      }

    rule pattern_list_headtail() -> Node<Pattern<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        [Token::BracketBegin]
        heads:pattern() ++ [Token::Comma]
        [Token::BitOr]
        tail:pattern()
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Pattern::ListHeadTail { heads, tail },
        )
      }

    rule pattern_eval() -> Node<Pattern<SourceLocation<'source>>, SourceLocation<'source>>
      = l:position!()
        [Token::DollarBraceBegin]
        expr:expression()
        [Token::BraceEnd]
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Pattern::Eval(expr),
        )
      }
    /* #endregion */

    /* #region Expression */
    rule expression() -> Node<Expression<SourceLocation<'source>>, SourceLocation<'source>>
      = precedence!{
          l:position!() data:@ r:position!() {
            Node::new(srcmap.token_span(l, r), data)
          }
          --
          lhs:pattern() [Token::Match] rhs:(@) {
            Expression::PatternBind { lhs, rhs }
          }
          --
          lhs:(@) [Token::TypeContains] rhs:type_() {
            Expression::TypeCheck { lhs, rhs, negated: false }
          }
          lhs:(@) [Token::TypeContains] [Token::LogicNot] rhs:type_() {
            Expression::TypeCheck { lhs, rhs, negated: true }
          }
          --
          lhs:(@) [Token::Pipe] rhs:expression_function_call() {
            match rhs {
              Expression::FunctionCall { name, type_params, mut call_params} => {
                call_params.insert(0, lhs);
                Expression::FunctionCall { name, type_params, call_params }
              },
              _ => unreachable!()
            }
          }
          --
          "throw" e:(@) {
            Expression::ExceptionThrow(e)
          }
          --
          lhs:(@) [Token::LogicOr] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::LogicOr, rhs }
          }
          --
          lhs:(@) [Token::LogicAnd] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::LogicAnd, rhs }
          }
          --
          lhs:(@) [Token::BitOr] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::BitOr, rhs }
          }
          --
          lhs:(@) [Token::BitXor] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::BitXor, rhs }
          }
          --
          lhs:(@) [Token::BitAnd] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::BitAnd, rhs }
          }
          --
          lhs:(@) [Token::ListContains] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::ListContains, rhs }
          }
          lhs:(@) [Token::LogicNot] [Token::ListContains] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::ListNotContains, rhs }
          }
          --
          lhs:(@) [Token::Eq] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::CompareEQ, rhs }
          }
          lhs:(@) [Token::Ne] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::CompareNE, rhs }
          }
          --
          lhs:(@) [Token::LT] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::CompareLT, rhs }
          }
          lhs:(@) [Token::LTE] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::CompareLTE, rhs }
          }
          lhs:(@) [Token::GTE] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::CompareGTE, rhs }
          }
          lhs:(@) [Token::GT] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::CompareGT, rhs }
          }
          --
          lhs:(@) [Token::BitLShift] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::BitLShift, rhs }
          }
          lhs:(@) [Token::BitRShift] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::BitRShift, rhs }
          }
          --
          lhs:(@) [Token::StringConcat] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::StringConcatenation, rhs }
          }
          --
          lhs:(@) [Token::ListConcat] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::ListConcatenation, rhs }
          }
          --
          lhs:(@) [Token::Add] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::ArithmeticAdd, rhs }
          }
          lhs:(@) [Token::Sub] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::ArithmeticSub, rhs }
          }
          --
          lhs:(@) [Token::Mul] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::ArithmeticMul, rhs }
          }
          lhs:(@) [Token::Div] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::ArithmeticDiv, rhs }
          }
          lhs:(@) [Token::Mod] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::ArithmeticMod, rhs }
          }
          --
          lhs:(@) [Token::Pow] rhs:@ {
            Expression::BinaryOperation { lhs, op: BinaryOperator::ArithmeticPow, rhs }
          }
          --
          [Token::Sub] e:(@) {
            Expression::UnaryOperation(UnaryOperator::ArithmeticNegation, e)
          }
          [Token::LogicNot] e:(@) {
            Expression::UnaryOperation(UnaryOperator::LogicNot, e)
          }
          [Token::BitNot] e:(@) {
            Expression::UnaryOperation(UnaryOperator::BitNot, e)
          }
          --
          lhs:(@) [Token::Dot] rhs:identifier() {
            Expression::MemberAccess { lhs, rhs }
          }
          --
          t:expression_term() { t }
          [Token::ParenBegin] e:expression() [Token::ParenEnd] { e.get_data().clone() }
        }

    rule expression_term() -> Expression<SourceLocation<'source>>
      = expression_literal()
      / expression_tuple()
      / expression_namedtuple()
      / expression_list()
      / expression_list_headtail()
      / expression_let()
      / expression_match()
      / expression_receive()
      / expression_cond()
      / expression_tailrec_recurse()
      / expression_tailrec_final()
      / expression_function_call()
      / expression_effect_call()
      / expression_process_spawn()
      / expression_block()
      / expression_variable()

    rule expression_literal() -> Expression<SourceLocation<'source>>
      = lit:literal()
      {
        Expression::Literal(lit)
      }

    rule expression_tuple() -> Expression<SourceLocation<'source>>
      = { todo!("expression_tuple") }

    rule expression_namedtuple() -> Expression<SourceLocation<'source>>
      = { todo!("expression_namedtuple") }

    rule expression_list() -> Expression<SourceLocation<'source>>
      = { todo!("expression_list") }

    rule expression_list_headtail() -> Expression<SourceLocation<'source>>
      = { todo!("expression_list_headtail") }

    rule expression_let() -> Expression<SourceLocation<'source>>
      = { todo!("expression_let") }

    rule expression_match() -> Expression<SourceLocation<'source>>
      = { todo!("expression_match") }

    rule expression_receive() -> Expression<SourceLocation<'source>>
      = { todo!("expression_receive") }

    rule expression_cond() -> Expression<SourceLocation<'source>>
      = { todo!("expression_cond") }

    rule expression_tailrec_recurse() -> Expression<SourceLocation<'source>>
      = { todo!("expression_tailrec_recurse") }

    rule expression_tailrec_final() -> Expression<SourceLocation<'source>>
      = { todo!("expression_tailrec_final") }

    rule expression_function_call() -> Expression<SourceLocation<'source>>
      = { todo!("expression_function_call") }

    rule expression_effect_call() -> Expression<SourceLocation<'source>>
      = { todo!("expression_effect_call") }

    rule expression_process_spawn() -> Expression<SourceLocation<'source>>
      = { todo!("expression_process_spawn") }

    rule expression_block() -> Expression<SourceLocation<'source>>
      = { todo!("expression_block") }

    rule expression_variable() -> Expression<SourceLocation<'source>>
      = ident:identifier()
      {
        Expression::Variable(ident)
      }

    /* #endregion */

    rule symbol_path() -> NonEmpty<Node<Identifier, SourceLocation<'source>>>
      = segments:(identifier() ++ [Token::ModSep])
      {
        NonEmpty::from_vec(segments).unwrap()
      }

    rule literal() -> Node<Literal, SourceLocation<'source>>
      = l:position!()
        lit:(
          (
            [Token::Atom(val)]
            { Literal::Atom(val.to_string()) }
          ) / (
            [Token::Bool(val)]
            { Literal::Boolean(*val) }
          ) / (
            [Token::Number(val)]
            { Literal::Number(*val) }
          ) / (
            [Token::String(val)]
            { Literal::String(val.clone()) }
          )
        )
        r:position!()
      {
        Node::new(srcmap.token_span(l, r), lit)
      }

    rule identifier() -> Node<Identifier, SourceLocation<'source>>
      = l:position!()
        [Token::Ident(ident)]
        r:position!()
      {
        Node::new(
          srcmap.token_span(l, r),
          Identifier(ident.to_string()),
        )
      }
  }
}
