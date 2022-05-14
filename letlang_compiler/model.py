class Model:
    def walk(self, node, *args, **kwargs):
        node_type = node.get("data", {}).get("_type")
        assert node_type is not None, "walk() expects an AST node"

        handler = getattr(self, f"walk_{node_type}", self.walk_default)
        return handler(node, *args, **kwargs)

    def walk_default(self, node, *_args, **_kwargs):
        raise NotImplementedError(node["data"]["_type"])
