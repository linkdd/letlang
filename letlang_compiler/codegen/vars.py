class VariablesMixin:
    def walk_Identifier(self, node, scope):
        var_name = node["data"]["identifier"]

        if var_name not in scope:
            raise NameError(f"Variable {var_name} not found in scope")

        symbol_type, sub_scope = scope[var_name]

        match symbol_type:
            case "arg":
                return f"paramval_{var_name}.clone()"

            case _:
                raise NotImplementedError(
                    f"cannot reference symbol of type {symbol_type} yet"
                )
