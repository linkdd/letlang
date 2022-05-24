import tomli_w


CODE_HEADER = """
#[allow(unused_variables)]
#[allow(dead_code)]

"""


class ModuleMixin:
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
                "async-trait": "0.1",
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
                dependencies=dependencies
            )
            tomli_w.dump(doc, f)

        return scope
