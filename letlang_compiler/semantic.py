import re

from .model import Model


UNIT_IDENTIFIER_RE = re.compile(r"^([a-z][a-z0-9]*)(\.[a-z][a-z0-9]*)*$")
UNIT_ALIAS_RE = re.compile(r"^([a-zA-Z_][a-zA-Z0-9_]*)")


class Semantic(Model):
    def walk_default(self, node, deps):
        pass

    def walk_Unit(self, node):
        unit_id = node["data"]["identifier"]

        if UNIT_IDENTIFIER_RE.match(unit_id) is None:
            raise ValueError(f"Invalid unit identifier: {unit_id}")

        deps = []

        for statement in node["data"]["statements"]:
            self.walk(statement, deps=deps)

        base_crate_name = unit_id.replace(".", "_")

        return (f"lldep_{base_crate_name}", deps)

    def walk_ImportStatement(self, node, deps):
        unit_id = node["data"]["path"]
        unit_alias = node["data"]["alias"]

        if UNIT_IDENTIFIER_RE.match(unit_id) is None:
            raise ValueError(f"Invalid unit identifier: {unit_id}")

        if unit_alias is not None and UNIT_ALIAS_RE.match(unit_alias) is None:
            raise ValueError(f"Invalid unit alias: {unit_alias}")

        base_crate_name = unit_id.replace(".", "_")
        deps.append(f"lldep_{base_crate_name}")

    def walk_FuncDeclStatement(self, node, deps):
        if node["data"]["type_params"]:
            raise NotImplementedError("Function generics not yet supported")

        if node["data"]["call_params"]:
            raise NotImplementedError("Function parameters not yet supported")
