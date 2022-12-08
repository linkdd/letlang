---
title: Abstract Syntax Tree
weight: 3
---

```typescript
unit = Unit(path: identifier+, body: statement*)

statement =
  | Import(path: identifier+, alias?: identifier)
  | EffectDeclaration(
      is_public: bool,
      symbol_name: identifier,
      call_params: call_param*,
      return_type: typeref,
    )
  | ClassDeclaration(
      is_public: bool,
      symbol_name: identifier,
      type_params: type_param*,
      cons_param: cons_param,
      constraints?: proposition+,
    )
  | FunctionDeclaration(
      is_public: bool,
      symbol_name: identifier,
      type_params: type_param*,
      call_params: call_param*,
      return_type: typeref,
      body: proposition+,
    )
  | TailRecursiveFunctionDeclaration(
      is_public: bool,
      symbol_name: identifier,
      type_params: type_param*,
      call_params: call_param*,
      return_type: typeref,
      body: proposition+,
    )

type_param = TypeParam(param_name: identifier)
call_param = CallParam(param_name: identifier, param_type: typeref)
cons_param = ConsParam(param_name: identifier, param_type: typeref)

typeref =
  | TypeValue(val: literal)
  | TypeName(symbol: identifier+, type_params: type_param*)
  | StructDefinition(members: typeref_struct_member*)
  | TupleDefinition(members: typeref*)
  | FunctionSignature(params: typeref*, return_type: typeref)
  | OneOf(alternatives: typeref*)
  | AllOf(alternatives: typeref*)
  | Not(typeref: typeref)

typeref_struct_member = StructMemberDefinition(name: identifier, typeref: typeref)

proposition =
  | Evaluation(expr: expression)
  | Constraint(symbol_name: identifier, symbol_type: typeref, checks: expression*)

expression =
  | Symbol(path: identifier+)
  | Literal(val: literal)
  | Structure(members: expression_struct_member*)
  | Tuple(members: expression*)
  | List(items: expression*)
  | EffectCall(name: identifier+, params: expression*)
  | FunctionCall(func: expression, params: expression*)
  | SpawnCall(func: expression, params: expression*)
  | GenericResolve(symbol_name: identifier+, type_params: typeref+)
  | MemberAccess(lhs: expression, rhs: identifier)
  | TypeCheck(lhs: expression, rhs: typeref, negate: bool)
  | UnaryOperation(op: unary_operator, expr: expression)
  | BinaryOperation(lhs: expression, op: binary_operator, rhs: expression)
  | PatternMatch(lhs: pattern, rhs: expression)
  | TailRecFinal(val: expression)
  | TailRecRecurse(args: expression*)
  | MatchBranching(expr: expression, cases: match_branch+)
  | ConditionalBranching(cases: cond_branch*, else_case: proposition+)
  | Receive(cases: receive_branch+, after?: receive_timeout_branch)

unary_operator = ...
binary_operator = ...

match_branch = MatchBranch(pattern: pattern, body: proposition+)
cond_branch = ConditionalBranch(cond: expression, body: proposition+)
receive_branch = ReceiveBranch(pattern: pattern, body: proposition+)
receive_timeout_branch = ReceiveTimeoutBranch(timeout: expression, body: proposition+)

pattern =
  | VariableBinding(symbol_name: identifier)
  | ValuePattern(val: expression)
  | LiteralPattern(val: literal)
  | TuplePattern(members: pattern*)
  | StructPattern(members: pattern_struct_member*)
  | ListPattern(items: pattern*)
  | ListHeadTailPattern(head: pattern, tail: pattern)

pattern_struct_member = StructMemberPattern(name: identifier, pattern: pattern)

literal =
  | BooleanLiteral(val: bool)
  | NumberLiteral(val: f64)
  | AtomLiteral(repr: string)
  | StringLiteral(val: string)

identifier = /[_a-zA-Z][_0-9a-zA-Z]*/
```