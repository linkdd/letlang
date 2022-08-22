mod cargo;
mod executable;
mod module;

use crate::prelude::*;
use letlang_parser::{
  ast,
  ast::Node,
};

use std::{path::{Path, PathBuf}, collections::HashMap};

pub type CodeGenResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct Context<'ctx> {
  pub atom_interner: &'ctx mut crate::phases::AtomInterner,
}

pub struct CodeGenerator<'ctx> {
  runtime_version: String,
  rust_edition: String,
  target_dir: PathBuf,
  context: Context<'ctx>,
}

impl<'ctx> CodeGenerator<'ctx> {
  pub fn new<P: AsRef<Path>>(
    runtime_version: String,
    rust_edition: String,
    target_dir: P,
    context: Context<'ctx>,
  ) -> Self {
    Self {
      runtime_version,
      rust_edition,
      target_dir: target_dir.as_ref().to_path_buf(),
      context,
    }
  }

  pub fn gen_lib_crate(
    &mut self,
    unit: &Node<ast::Unit>,
    version: &str,
  ) -> CodeGenResult<String> {
    let crate_name = format!("lldep_{}", unit.data.path.join("_"));

    let mut folder_path = PathBuf::new();
    folder_path.push(&self.target_dir);
    folder_path.push("modules");
    folder_path.push(&crate_name);
    std::fs::create_dir_all(&folder_path)?;

    let mut crate_cargo_path = PathBuf::new();
    crate_cargo_path.push(&folder_path);
    crate_cargo_path.push("Cargo.toml");

    let mut crate_deps: HashMap<String, cargo::CargoDependencyConfig> = HashMap::new();
    self.inject_runtime_dep(&mut crate_deps);

    let crate_cargo_cfg = cargo::UnitConfig {
      package: cargo::CargoPackageConfig {
        name: crate_name.clone(),
        version: version.to_string(),
        edition: self.rust_edition.clone(),
      },
      dependencies: crate_deps,
    };
    std::fs::write(crate_cargo_path, crate_cargo_cfg.to_string())?;

    let mut source_dir = PathBuf::new();
    source_dir.push(&folder_path);
    source_dir.push("src");
    std::fs::create_dir_all(&source_dir)?;

    let mut lib_path = PathBuf::new();
    lib_path.push(&source_dir);
    lib_path.push("lib.rs");

    let source_code = self.gen_lib_source(unit)?;
    std::fs::write(&lib_path, source_code)?;
    self.reformat_source(&lib_path)?;

    Ok(crate_name)
  }

  pub fn gen_exe_crate(
    &mut self,
    bin: &str,
    main_module: &str,
    version: &str,
  ) -> CodeGenResult<()> {
    let main_crate = format!("lldep_{}", main_module.replace("::", "_"));

    let mut folder_path = PathBuf::new();
    folder_path.push(&self.target_dir);
    folder_path.push("executable");
    std::fs::create_dir_all(&folder_path)?;

    let mut crate_cargo_path = PathBuf::new();
    crate_cargo_path.push(&folder_path);
    crate_cargo_path.push("Cargo.toml");

    let mut crate_deps: HashMap<String, cargo::CargoDependencyConfig> = HashMap::new();
    self.inject_runtime_dep(&mut crate_deps);

    crate_deps.insert(
      main_crate.clone(),
      cargo::CargoDependencyConfig::FileSystem {
        path: format!("../modules/{}", main_crate.clone()),
      }
    );

    let crate_cargo_cfg = cargo::UnitConfig {
      package: cargo::CargoPackageConfig {
        name: bin.to_string(),
        version: version.to_string(),
        edition: self.rust_edition.clone(),
      },
      dependencies: crate_deps,
    };
    std::fs::write(crate_cargo_path, crate_cargo_cfg.to_string())?;

    let mut source_dir = PathBuf::new();
    source_dir.push(&folder_path);
    source_dir.push("src");
    std::fs::create_dir_all(&source_dir)?;

    let mut exe_path = PathBuf::new();
    exe_path.push(&source_dir);
    exe_path.push("main.rs");

    let source_code = self.gen_exe_source(&main_crate)?;
    std::fs::write(&exe_path, source_code)?;
    self.reformat_source(&exe_path)?;

    Ok(())
  }

  pub fn gen_workspace(&mut self, members: Vec<String>) -> CodeGenResult<()> {
    let mut cargo_path = PathBuf::new();
    cargo_path.push(&self.target_dir);
    cargo_path.push("Cargo.toml");

    let workspace_cargo_cfg = cargo::ProjectConfig {
      workspace: cargo::CargoWorkspaceConfig { members }
    };
    std::fs::write(cargo_path, toml::to_string(&workspace_cargo_cfg)?)?;

    Ok(())
  }

  fn gen_lib_source(&mut self, unit: &Node<ast::Unit>) -> CompilationResult<String> {
    let generator = module::CodeGenerator::new(&self.context);
    generator.generate(unit)
  }

  fn gen_exe_source(&mut self, crate_name: &str) -> CompilationResult<String> {
    let mut generator = executable::CodeGenerator::new(&mut self.context);
    generator.generate(crate_name)
  }

  fn reformat_source<P: AsRef<Path>>(&mut self, path: P) -> CodeGenResult<()> {
    let status = std::process::Command::new("rustfmt")
      .arg(format!("--edition={}", self.rust_edition))
      .arg(path.as_ref().to_path_buf())
      .status()?;

    if !status.success() {
      eprintln!("WARN: rustfmt command exited with non-zero code: {:?}", status.code());
    }

    Ok(())
  }

  fn inject_runtime_dep(&self, deps: &mut HashMap<String, cargo::CargoDependencyConfig>) {
    let dep = match std::env::var("LETLANG_RUNTIME_PATH") {
      Ok(val) => {
        match val.trim() {
          "" => {
            cargo::CargoDependencyConfig::Versionned(self.runtime_version.clone())
          },
          path => {
            cargo::CargoDependencyConfig::FileSystem {
              path: path.to_string(),
            }
          },
        }
      },
      Err(_) => {
        cargo::CargoDependencyConfig::Versionned(self.runtime_version.clone())
      },
    };

    deps.insert(String::from("llcore_runtime"), dep);
  }
}
