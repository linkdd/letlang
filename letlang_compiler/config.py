from pathlib import Path

import tomli
import cattr
import attr


@attr.define
class Package:
    name: str
    version: str
    description: str


@attr.define
class Executable:
    bin: str
    module: str


@attr.define
class Toolchain:
    letlang: str
    rust: str


@attr.define
class LocalDependency:
    path: str


@attr.define
class GitDependency:
    git_url: str
    git_ref: str | None


@attr.define
class Config:
    package: Package
    executable: Executable | None
    toolchain: Toolchain
    dependencies: dict[str, str | LocalDependency | GitDependency]

    @classmethod
    def load_from_file(cls, path: str | None) -> tuple[Path, "Config"]:
        if path is None:
            path = Path.cwd() / "letproject.toml"

        else:
            path = Path(path).absolute()

        with path.open(mode="rb") as f:
            data = tomli.load(f)

        return (path.parent, cattr.structure(data, cls))
