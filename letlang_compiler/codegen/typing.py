BUILTIN_TYPES = [
    "bool",
    "int",
    "number",
    "string",
    "atom",
]


class TypingMixin:
    def walk_TypeCheck(self, node, scope):
        val = self.walk(node["data"]["lhs"], scope=scope)
        typeref = self.walk(node["data"]["rhs"], scope=scope)

        template = self.get_template("typecheck_expression.rs.j2")
        return template.render(val=val, typeref=typeref)

    def walk_ValueType(self, node, scope):
        primitive_code = self.walk(node["data"], scope=scope)
        return f"types::ValueType {{ llval: {primitive_code} }}"

    def walk_AllOfType(self, node, scope):
        typerefs = [
            self.walk(typeref, scope=scope)
            for typeref in node["data"]["typerefs"]
        ]

        template = self.get_template("allof_type.rs.j2")
        return template.render(typerefs=typerefs)

    def walk_OneOfType(self, node, scope):
        typerefs = [
            self.walk(typeref, scope=scope)
            for typeref in node["data"]["typerefs"]
        ]

        template = self.get_template("oneof_type.rs.j2")
        return template.render(typerefs=typerefs)

    def walk_NotType(self, node, scope):
        typeref = self.walk(node["data"]["typeref"], scope=scope)

        template = self.get_template("not_type.rs.j2")
        return template.render(typeref=typeref)

    def walk_TypeName(self, node, scope):
        type_name = node["data"]["name"]

        if type_name in scope:
            item_type, sub_scope = scope[type_name]

            if item_type == "class":
                return f"crate::symbol_{type_name}::class_{type_name} {{}}"

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
