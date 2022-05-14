import tomli_w


class ExecutableMixin:
    def gen_exe(self, main_crate: str):
        target_path = self.target_dir / "exe" / self.config.executable.bin
        rust_file_path = target_path / "src" / "main.rs"
        cargo_file_path = target_path / "Cargo.toml"

        rust_file_path.parent.mkdir(parents=True, exist_ok=True)
        template = self.get_template("exe_entrypoint.rs.j2")
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
