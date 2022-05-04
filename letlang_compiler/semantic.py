import re

from .model import Model


UNIT_IDENTIFIER_RE = re.compile(r"^([a-z][a-z0-9]*)(\.[a-z][a-z0-9]*)*$")


class Semantic(Model):
    def walk_default(self, node):
        pass

    def walk_Unit(self, node):
        unit_id = node["data"]["identifier"]

        if UNIT_IDENTIFIER_RE.match(unit_id) is None:
            raise ValueError(f"Invalid unit identifier: {unit_id}")

        for statement in node["data"]["statements"]:
            self.walk(statement)

    def walk_ImportStatement(self, node):
        unit_id = node["data"]["path"]

        if UNIT_IDENTIFIER_RE.match(unit_id) is None:
            raise ValueError(f"Invalid unit identifier: {unit_id}")
