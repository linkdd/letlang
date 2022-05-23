BUILTIN_TYPES = [
    "bool",
    "int",
    "number",
    "string",
    "atom",
]


class TypingMixin:
    def walk_ValueType(self, node, scope):
        primitive_code = self.walk(node["data"], scope=scope)
        return f"types::ValueType {{ llval: {primitive_code} }}"

    def walk_TypeName(self, node, scope):
        type_name = node["data"]["name"]

        if type_name in scope:
            item_type, sub_scope = scope[type_name]

            if item_type == "type":
                raise NotImplementedError("Custom class not supported yet")

        elif type_name in BUILTIN_TYPES:
            match type_name:
                case "bool":
                    return "types::BooleanType {}"

                case "int":
                    return "types::IntegerType {}"

                case "number":
                    return "types::NumberType {}"

                case "string":
                    return "types::StringType {}"

                case "atom":
                    return "types::AtomType {}"

        else:
            raise NameError(f"Type {type_name} not found in scope")
