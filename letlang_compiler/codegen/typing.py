class TypingMixin:
    def walk_ValueType(self, node, scope):
        primitive_code = self.walk(node["data"], scope=scope)
        return f"types::ValueType {{ llval: {primitive_code} }}"
