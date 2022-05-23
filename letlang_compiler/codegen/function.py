import json


class FunctionMixin:
    def walk_CallParam(self, node, scope):
        param_name = node["data"]["param_name"]
        scope[param_name] = ("arg", {})
        param_type = self.walk(node["data"]["param_type"], scope=scope)

        return dict(name=param_name, type_code=param_type)

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

        call_params = [
            self.walk(call_param, scope=scope)
            for call_param in node["data"]["call_params"]
        ]

        template = self.get_template("function_call_expression.rs.j2")
        return template.render(
            func_name=func_name,
            call_params=call_params,
        )

    def walk_EffectCall(self, node, scope):
        if node["data"]["effect_name"] != "debug":
            raise NotImplementedError("only debug side effect is supported for now")

        call_params = [
            self.walk(call_param, scope=scope)
            for call_param in node["data"]["call_params"]
        ]

        template = self.get_template("trigger_side_effect.rs.j2")
        return template.render(
            effect_name=json.dumps(node["data"]["effect_name"]),
            call_params=call_params,
        )
