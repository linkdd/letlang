import json


class LiteralMixin:
    def walk_Literal(self, node, scope):
        primitive_code = self.walk(node["data"], scope=scope)
        return f"Value::Primitive({primitive_code})"

    def walk_Number(self, node, scope):
        value = node["data"]["value"]
        return f"PrimitiveValue::Number({value})"

    def walk_Atom(self, node, scope):
        value = node["data"]["value"]
        return f"PrimitiveValue::Atom(context.lock().unwrap().get_atom({json.dumps(value)}))"

    def walk_String(self, node, scope):
        value = node["data"]["value"]
        return f"PrimitiveValue::String({json.dumps(value)}.to_string())"
