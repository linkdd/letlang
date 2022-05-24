from copy import deepcopy


class StatementMixin:
    def walk_ImportStatement(self, node, scope):
        unit_id = node["data"]["path"].replace(".", "_")
        unit_alias = node["data"]["alias"]

        if unit_alias is None:
            unit_alias = node["data"]["path"].split(".")[-1]

        scope[unit_alias] = ("module", {})

        template = self.get_template("import_statement.rs.j2")
        return template.render(
            unit_id=unit_id,
            unit_alias=unit_alias,
        )

    def walk_ConstDeclStatement(self, node, scope):
        public = node["data"]["public"]
        symbol_name = node["data"]["symbol_name"]
        value_code = self.walk(node["data"]["value"], scope=scope)

        scope[symbol_name] = ("const", {})

        template = self.get_template("const_decl_statement.rs.j2")
        return template.render(
            public=public,
            symbol_name=symbol_name,
            value_code=value_code,
        )

    def walk_ClassDeclStatement(self, node, scope):
        class_scope = deepcopy(scope)

        public = node["data"]["public"]
        symbol_name = node["data"]["symbol_name"]
        type_params = []
        cons_param = self.walk(node["data"]["cons_param"], scope=class_scope)
        constraints = [
            self.walk(constraint, scope=class_scope)
            for constraint in node["data"]["constraints"]
        ]

        scope[symbol_name] = ("class", class_scope)

        template = self.get_template("class_decl_statement.rs.j2")
        return template.render(
            public=public,
            symbol_name=symbol_name,
            type_params=type_params,
            cons_param=cons_param,
            constraints=constraints,
        )

    def walk_ConsParam(self, node, scope):
        param_name = node["data"]["param_name"]
        scope[param_name] = ("arg", {})
        param_type = self.walk(node["data"]["param_type"], scope=scope)

        return dict(name=param_name, type_code=param_type)

    def walk_FuncDeclStatement(self, node, scope):
        func_scope = deepcopy(scope)

        public = node["data"]["public"]
        symbol_name = node["data"]["symbol_name"]
        type_params = []
        call_params = [
            self.walk(call_param, scope=func_scope)
            for call_param in node["data"]["call_params"]
        ]
        return_type = self.walk(node["data"]["return_type"], scope=func_scope)

        scope[symbol_name] = ("func", func_scope)
        func_scope[symbol_name] = ("func", func_scope)

        body = [
            self.walk(expr, scope=func_scope)
            for expr in node["data"]["body"]
        ]

        template = self.get_template("func_decl_statement.rs.j2")
        return template.render(
            public=public,
            symbol_name=symbol_name,
            type_params=type_params,
            call_params=call_params,
            return_type=return_type,
            body=body,
        )
