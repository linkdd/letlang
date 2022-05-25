OPERATIONS = {
    "+": "operations::add",
    "-": "operations::sub",
    "*": "operations::mul",
    "/": "operations::div",
    "%": "operations::modulo",
    "=": "operations::eq",
    "!=": "operations::ne",
    "<": "operations::lt",
    "<=": "operations::lte",
    ">=": "operations::gte",
    ">": "operations::gt",
    "and": "operations::and",
    "or": "operations::or",
    "==>": "operations::imply",
    "<==>": "operations::bicond",
}

class BinaryOperationMixin:
    def walk_BinaryOperation(self, node, scope):
        lhs = self.walk(node["data"]["lhs"], scope=scope)
        rhs = self.walk(node["data"]["rhs"], scope=scope)
        op = node["data"]["op"]

        if op not in OPERATIONS:
            raise NotImplementedError(f"Unary Operator {op} not yet supported")

        template = self.get_template("binop_expression.rs.j2")
        return template.render(
            lhs=lhs,
            rhs=rhs,
            op=op,
            fn_op=OPERATIONS[op],
        )
