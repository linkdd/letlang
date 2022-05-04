class Model:
    def walk(self, node):
        node_type = node.get("data", {}).get("_type")
        assert node_type is not None, "walk() expects an AST node"

        handler = getattr(self, f"walk_{node_type}", self.walk_default)
        return handler(node)

    def walk_default(self, node):
        raise NotImplementedError(node["data"]["_type"])
