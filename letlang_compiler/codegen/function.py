class FunctionMixin:
    def walk_FunctionCall(self, node, scope):
        func_path = node["data"]["func_name"].split(".")
        func_scope, func_name = func_path[:-1], func_path[-1]

        cur_scope = scope
        for scope_name in func_scope:
            if scope_name not in cur_scope:
                raise NameError(f"Name {scope_name} not found in scope")

            scope_type, sub_scope = scope[scope_name]

            match scope_type:
                case "module":
                    cur_scope = sub_scope

                case _:
                    raise ValueError(f"{scope_name} is not traversable")

        if func_name not in cur_scope:
            raise NameError(f"Function {func_name} not found in scope")

        assert False
