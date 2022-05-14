from typing import Iterator
from pathlib import Path
import subprocess
import json

import networkx as nx
import click

from letlang_compiler.config import Config
from letlang_compiler.semantic import Semantic
from letlang_compiler.codegen import CodeGen
import letlang_parser


def parse(source_file_paths: Iterator[Path]) -> list[dict]:
    ast = []

    label = "Parsing   "
    with click.progressbar(source_file_paths, label=label) as iterator:
        for source_file_path in iterator:
            json_ast = letlang_parser.parse(str(source_file_path))
            ast.append(json.loads(json_ast))

    return ast


def validate(
    ast: list[dict],
    project_cfg: Config,
) -> tuple[nx.DiGraph, dict[str, dict]]:
    G = nx.DiGraph()
    crates = {}
    edges = []

    label = "Validating"
    with click.progressbar(ast, label=label) as iterator:
        for unit_ast in iterator:
            model = Semantic()
            crate_name, deps = model.walk(unit_ast)

            G.add_node(crate_name)
            crates[crate_name] = unit_ast

            for dep in deps:
                edges.append((crate_name, dep))

    for source, target in edges:
        if target not in G:
            raise ImportError(f"No module {target} found")

        G.add_edge(source, target)

    if project_cfg.executable is not None:
        main_unit_id = project_cfg.executable.module
        base_crate_name = main_unit_id.replace(".", "_")
        crate_name = f"lldep_{base_crate_name}"

        if crate_name not in G:
            raise ImportError(f"Main module {main_unit_id} not found")

    return (G, crates)


def compile(
    crates_ast: dict[str, dict],
    dep_graph: nx.DiGraph,
    project_cfg: Config,
    target_dir: Path,
):
    scopes = {}

    label = "Compiling "
    with click.progressbar(crates_ast.keys(), label=label) as iterator:
        for crate_name in iterator:
            unit_ast = crates_ast[crate_name]
            dependencies = list(dep_graph.successors(crate_name))
            model = CodeGen(project_cfg, target_dir, dependencies)
            scopes[crate_name] = model.walk(unit_ast)

        if project_cfg.executable is not None:
            main_unit_id = project_cfg.executable.module
            base_crate_name = main_unit_id.replace(".", "_")
            crate_name = f"lldep_{base_crate_name}"
            scope = scopes[crate_name]

            if "main" not in scope or scope["main"][0] != "func":
                raise ValueError(f"No function main found in {main_unit_id} module")

            dependencies = [crate_name]
            model = CodeGen(project_cfg, target_dir, dependencies)
            model.gen_exe(crate_name)

        model = CodeGen(project_cfg, target_dir, [])
        members = []

        for crate_name in crates_ast.keys():
            prefix = "lldep_"
            crate_folder = crate_name[len(prefix):]
            members.append(f"modules/{crate_folder}")

        if project_cfg.executable is not None:
            members.append(f"exe/{project_cfg.executable.bin}")

        model.gen_project(members)


def assemble(target_dir: Path, command="build"):
    subprocess.run(f"cargo {command}", shell=True, check=True, cwd=target_dir)
