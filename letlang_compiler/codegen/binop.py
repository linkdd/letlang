class BinaryOperationMixin:
    def walk_BinaryOperation(self, node, scope):
        lhs = self.walk(node["data"]["lhs"], scope=scope)
        rhs = self.walk(node["data"]["rhs"], scope=scope)
        op = node["data"]["op"]

        match op:
            case "+":
                fn_op = "operations::add"

            case "-":
                fn_op = "operations::sub"

            case "%":
                fn_op = "operations::modulo"

            case "=":
                fn_op = "operations::equal"

            case op:
                raise NotImplementedError(f"Operator {op} not yet supported")

        template = self.get_template("binop_expression.rs.j2")
        return template.render(
            lhs=lhs,
            rhs=rhs,
            op=op,
            fn_op=fn_op,
        )
