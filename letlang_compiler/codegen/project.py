import tomli_w


class ProjectMixin:
    def gen_project(self, members: list[str]):
        cargo_file_path = self.target_dir / "Cargo.toml"

        cargo_file_path.parent.mkdir(parents=True, exist_ok=True)

        with cargo_file_path.open(mode="wb") as f:
            doc = dict(workspace=dict(members=members))
            tomli_w.dump(doc, f)
