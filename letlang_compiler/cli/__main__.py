import click
import sys

from pathlib import Path
import json

from letlang_compiler.config import Config
from . import build as build_steps


@click.group()
@click.option("-p", "--project-path", default=None, help="Path to letproject.toml")
@click.pass_context
def cli(ctx, project_path):
    """Letlang Compiler"""

    project_dir, project_cfg = Config.load_from_file(project_path)

    ctx.ensure_object(dict)
    ctx.obj["project_cfg"] = project_cfg
    ctx.obj["project_dir"] = project_dir
    ctx.obj["source_dir"] = project_dir / "src"
    ctx.obj["target_dir"] = project_dir / ".llbuild"


@cli.command()
@click.pass_context
def build(ctx):
    """Build Letlang project"""

    project_cfg: Config = ctx.obj["project_cfg"]
    source_dir: Path = ctx.obj["source_dir"]
    target_dir: Path = ctx.obj["target_dir"]

    source_file_paths = source_dir.glob("**/*.let")
    ast = build_steps.parse(source_file_paths)

    with open("ast.json", "w") as f:
        json.dump(ast, f, indent=4)

    graph, crates_ast = build_steps.validate(ast, project_cfg)
    build_steps.compile(crates_ast, graph, project_cfg, target_dir)


@cli.command()
@click.pass_context
def run(ctx):
    """Build and run Letlang project"""

    build(ctx)


def main():
    try:
        cli(obj={})

    except Exception as err:
        if False:
            click.echo(f"{type(err).__name__}: {err}", file=sys.stderr)
            sys.exit(1)

        else:
            raise err


if __name__ == "__main__":
    main()
