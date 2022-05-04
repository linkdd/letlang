import click
import sys

from pathlib import Path
import json

from .config import Config

from .semantic import Semantic
from .codegen import CodeGen
import letlang_parser


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

    source_dir: Path = ctx.obj["source_dir"]
    target_dir: Path = ctx.obj["target_dir"]

    source_file_paths = source_dir.glob("**/*.let")
    ast = {}
    rust_files = {}

    label = "Parsing   "
    with click.progressbar(source_file_paths, label=label) as iterator:
        for source_file_path in iterator:
            json_ast = letlang_parser.parse(str(source_file_path))
            ast[source_file_path] = json.loads(json_ast)

    label = "Validating"
    with click.progressbar(ast.items(), label=label) as iterator:
        for _, unit_ast in iterator:
            model = Semantic()
            model.walk(unit_ast)

    label = "Compiling "
    with click.progressbar(ast.items(), label=label) as iterator:
        for _, unit_ast in iterator:
            model = CodeGen(target_dir)
            crate_name, rust_file_path = model.walk(unit_ast)
            rust_files[crate_name] = rust_file_path


@cli.command()
@click.pass_context
def run(ctx):
    """Build and run Letlang project"""

    build(ctx)


def main():
    try:
        cli(obj={})

    except Exception as err:
        click.echo(f"{type(err).__name__}: {err}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
