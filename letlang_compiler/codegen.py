from dataclasses import dataclass
from pathlib import Path

from jinja2 import Environment, PackageLoader

from .model import Model
from .utils import hashfile


env = Environment(
    loader=PackageLoader("letlang_compiler"),
    autoescape=False,
)

CODE_HEADER = """\
extern crate llcore_runtime;
use llcore_runtime::*;
"""


@dataclass
class CodeGen(Model):
    target_dir: Path

    def walk_Unit(self, node):
        filehash = hashfile(node["filename"])
        prefix, target_hash = filehash[:2], filehash[2:]

        target_path = self.target_dir / "rs" / prefix / f"{target_hash}.rs"

        if not target_path.exists():
            target_path.parent.mkdir(parents=True, exist_ok=True)

            code = CODE_HEADER

            for statement in node["data"]["statements"]:
                code += "\n"
                code += self.walk(statement)

            with target_path.open(mode="w") as f:
                f.write(code)

        unit_id = node["data"]["identifier"].replace(".", "_")

        return (f"lldep_{unit_id}", target_path)

    def walk_ImportStatement(self, node):
        unit_id = node["data"]["path"].replace(".", "_")

        template = env.get_template("import_statement.rs.j2")
        return template.render(unit_id=unit_id)

    def walk_ConstDeclStatement(self, node):
        symbol_name = node["data"]["symbol_name"]
        value_code = self.walk(node["data"]["value"])

        template = env.get_template("const_decl_statement.rs.j2")
        return template.render(
            symbol_name=symbol_name,
            value_code=value_code,
        )

    def walk_Literal(self, node):
        return self.walk(node["data"])

    def walk_Number(self, node):
        value = node["data"]["value"]
        return f"Value::from({value})"
