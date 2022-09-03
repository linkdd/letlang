use letlang_cli::config::ProjectConfig;
use letlang_compiler::{Compiler, Toolchain, Target};

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[clap(propagate_version=true)]
struct Cli {
  #[clap(short, long, value_parser)]
  project_path: Option<PathBuf>,
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  Build,
  Run,
}

fn main() -> Result<()> {
  let cli = Cli::parse();

  let project_dir = std::fs::canonicalize(
    cli.project_path.unwrap_or(PathBuf::from("."))
  )?;
  let project_config = load_config(&project_dir)?;

  match cli.command {
    Commands::Build => {
      build_project(&project_dir, &project_config)?;
      cargo_command(&project_dir, "build")?;
    },
    Commands::Run => {
      build_project(&project_dir, &project_config)?;
      cargo_command(&project_dir, "run")?;
    }
  }

  Ok(())
}

fn load_config<P: AsRef<Path>>(project_dir: P) -> Result<ProjectConfig>
{
  let mut project_path = PathBuf::new();
  project_path.push(project_dir);
  project_path.push("letproject.toml");
  ProjectConfig::load(project_path)
}

fn build_project<P: AsRef<Path>>(project_dir: P, cfg: &ProjectConfig) -> Result<()>
{
  let mut inputs = glob_sources(&project_dir);

  for (dep_name, dep_config) in cfg.dependencies.iter() {
    let mut dep_path = PathBuf::new();
    dep_path.push(&project_dir);
    dep_path.push(&dep_config.path);

    if !dep_path.exists() {
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("dependency {} not found at path {}", dep_name, dep_path.display())
      )));
    }

    if !dep_path.is_dir() {
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("dependency {} at path {} is not a directory", dep_name, dep_path.display())
      )));
    }

    inputs.append(&mut glob_sources(&dep_path));
  }

  let target_dir = make_target_dir(&project_dir)?;

  let mut compiler = Compiler::new(
    Toolchain {
      runtime_version: cfg.toolchain.letlang.clone(),
      rust_edition: cfg.toolchain.rust.clone(),
    },
    Target {
      binary_name: cfg.executable.bin.clone(),
      main_module: cfg.executable.module.clone(),
      package_version: cfg.package.version.clone(),
    },
  );
  compiler.compile(inputs, target_dir)?;

  Ok(())
}

fn cargo_command<P: AsRef<Path>>(project_dir: P, cmd: &str) -> Result<()> {
  let target_dir = make_target_dir(&project_dir)?;

  let status = std::process::Command::new("cargo")
    .current_dir(target_dir)
    .arg(cmd)
    .status()?;

  if !status.success() {
    eprintln!("ERROR: cargo command returned non-zero exit code: {:?}", status.code());
    std::process::exit(1);
  }

  Ok(())
}

fn glob_sources<P: AsRef<Path>>(project_dir: P) -> Vec<PathBuf>
{
  let mut source_dir = PathBuf::new();
  source_dir.push(project_dir);
  source_dir.push("src");

  let paths: Vec<PathBuf> = walkdir::WalkDir::new(source_dir)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
    .filter(|e| {
      if let Some(ext) = e.path().extension() {
        match ext.to_str() {
          Some("let") => true,
          _ => false
        }
      }
      else {
        false
      }
    })
    .map(|e| e.path().to_path_buf())
    .collect();

  paths
}

fn make_target_dir<P: AsRef<Path>>(project_dir: P) -> Result<PathBuf> {
  let mut target_dir = PathBuf::new();
  target_dir.push(project_dir);
  target_dir.push(".llbuild");

  std::fs::create_dir_all(&target_dir)?;

  Ok(target_dir)
}