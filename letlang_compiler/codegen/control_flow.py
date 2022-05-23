class ControlFlowMixin:
    def walk_FlowMatch(self, node, scope):
        expression_code = self.walk(node["data"]["value"], scope=scope)
        clauses = [
            self.walk(clause, scope=scope)
            for clause in node["data"]["clauses"]
        ]

        template = self.get_template("match_expression.rs.j2")
        return template.render(
            expression=expression_code,
            clauses=clauses,
        )

    def walk_FlowMatchClause(self, node, scope):
        pattern = self.walk(node["data"]["pattern"], scope=scope)
        body = [
            self.walk(expression, scope=scope)
            for expression in node["data"]["body"]
        ]

        return dict(pattern=pattern, body=body)
