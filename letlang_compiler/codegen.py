from dataclasses import dataclass
from copy import deepcopy
from pathlib import Path
import tomli_w
import json

from jinja2 import Environment, PackageLoader

from .config import Config
from .model import Model


env = Environment(
    loader=PackageLoader("letlang_compiler"),
    autoescape=False,
)

CODE_HEADER = """
#[allow(unused_variables)]
#[allow(dead_code)]

"""


@dataclass
class CodeGen(Model):
    config: Config
    target_dir: Path
    dependencies: list[str]

    def gen_exe(self, main_crate: str):
        target_path = self.target_dir / "exe" / self.config.executable.bin
        rust_file_path = target_path / "src" / "main.rs"
        cargo_file_path = target_path / "Cargo.toml"

        rust_file_path.parent.mkdir(parents=True, exist_ok=True)
        template = env.get_template("exe_entrypoint.rs.j2")
        code = template.render(main_crate=main_crate)

        with rust_file_path.open(mode="w") as f:
            f.write(code)

        with cargo_file_path.open(mode="wb") as f:
            prefix = "lldep_"
            main_crate_folder = main_crate[len(prefix):]

            doc = dict(
                package=dict(
                    name=self.config.executable.bin,
                    version=self.config.package.version,
                    edition=self.config.toolchain.rust,
                ),
                workspace=dict(),
                dependencies={
                    "llcore_runtime": dict(
                        path="../../../../../letlang_runtime",
                        package="letlang_runtime",
                    ),
                    main_crate: dict(
                        path=f"../../modules/{main_crate_folder}",
                        package=main_crate,
                    )
                }
            )
            tomli_w.dump(doc, f)

    def walk_Unit(self, node):
        unit_id = node["data"]["identifier"].replace(".", "_")

        target_path = self.target_dir / "modules" / unit_id
        rust_file_path = target_path / "src" / "lib.rs"
        cargo_file_path = target_path / "Cargo.toml"

        rust_file_path.parent.mkdir(parents=True, exist_ok=True)

        code = CODE_HEADER
        scope = {}

        for statement in node["data"]["statements"]:
            code += "\n"
            code += self.walk(statement, scope=scope)

        with rust_file_path.open(mode="w") as f:
            f.write(code)

        with cargo_file_path.open(mode="wb") as f:
            dependencies = {
                "llcore_runtime": dict(
                    path="../../../../../letlang_runtime",
                    package="letlang_runtime",
                ),
                "genawaiter": "0.99",
                "string-interner": "0.14",
            }

            for dep_name in self.dependencies:
                prefix = "lldep_"
                crate_folder = dep_name[len(prefix):]
                dependencies[dep_name] = dict(
                    path=f"../{crate_folder}",
                    package=dep_name,
                )

            doc = dict(
                package=dict(
                    name=f"lldep_{unit_id}",
                    version=self.config.package.version,
                    edition=self.config.toolchain.rust,
                ),
                workspace=dict(),
                dependencies=dependencies
            )
            tomli_w.dump(doc, f)

        return scope

    def walk_ImportStatement(self, node, scope):
        unit_id = node["data"]["path"].replace(".", "_")
        unit_alias = node["data"]["alias"]

        if unit_alias is None:
            unit_alias = node["data"]["path"].split(".")[-1]

        scope[unit_alias] = ("module", {})

        template = env.get_template("import_statement.rs.j2")
        return template.render(
            unit_id=unit_id,
            unit_alias=unit_alias,
        )

    def walk_ConstDeclStatement(self, node, scope):
        public = node["data"]["public"]
        symbol_name = node["data"]["symbol_name"]
        value_code = self.walk(node["data"]["value"], scope=scope)

        scope[symbol_name] = ("const", {})

        template = env.get_template("const_decl_statement.rs.j2")
        return template.render(
            public=public,
            symbol_name=symbol_name,
            value_code=value_code,
        )

    def walk_FuncDeclStatement(self, node, scope):
        public = node["data"]["public"]
        symbol_name = node["data"]["symbol_name"]
        type_params = []
        call_params = []
        return_type = self.walk(node["data"]["return_type"], scope=scope)

        func_scope = deepcopy(scope)
        scope[symbol_name] = ("func", func_scope)
        body = [
            self.walk(expr, scope=func_scope)
            for expr in node["data"]["body"]
        ]

        template = env.get_template("func_decl_statement.rs.j2")
        return template.render(
            public=public,
            symbol_name=symbol_name,
            type_params=type_params,
            call_params=call_params,
            return_type=return_type,
            body=body,
        )

    def walk_ValueType(self, node, scope):
        primitive_code = self.walk(node["data"], scope=scope)
        return f"types::ValueType {{ llval: {primitive_code} }}"

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

    def walk_FunctionCall(self, node, scope):
        func_path = node["data"]["func_name"].split(".")
        func_scope, func_name = func_path[:-1], func_path[-1]

        cur_scope = scope
        for scope_name in func_scope:
            if scope_name not in cur_scope:
                raise NameError(f"Name {scope_name} not found in scope")

            scope_type, sub_scope = scope[scope_name]

            match scope_type:
                case "module":
                    cur_scope = sub_scope

                case _:
                    raise ValueError(f"{scope_name} is not traversable")

        if func_name not in cur_scope:
            raise NameError(f"Function {func_name} not found in scope")

        assert False
