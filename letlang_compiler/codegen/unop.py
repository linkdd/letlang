OPERATIONS = {
    "-": "operations::neg",
    "not": "operations::not",
}

class UnaryOperationMixin:
    def walk_UnaryOperation(self, node, scope):
        expression = self.walk(node["data"]["expr"], scope=scope)
        op = node["data"]["op"]

        if op not in OPERATIONS:
            raise NotImplementedError(f"Unary Operator {op} not yet supported")

        template = self.get_template("unop_expression.rs.j2")
        return template.render(
            expression=expression,
            op=op,
            fn_op=OPERATIONS[op],
        )
