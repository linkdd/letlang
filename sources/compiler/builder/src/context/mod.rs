use std::{
  collections::HashMap,
  path::PathBuf,
  fs::{create_dir_all, File},
  io::{copy, prelude::*},
  process::{Command, Stdio},
  str,
};

use anyhow::{Context, Result};
use ar::Archive;

use crate::cargo::*;
use llfront::*;
use llbmi::BinaryModuleInterface;

const ERROR_TARGET_DIR: &str = "Failed to create target directory";
const ERROR_DEP_ARCHIVE: &str = "Failed to extract dependency archive";
const ERROR_BUILD_ARTIFACT: &str = "Failed to build artifact";
const ERROR_WRITE_MANIFEST: &str = "Failed to create artifact's manifest";
const ERROR_WRITE_SOURCE: &str = "Failed to write generated code";
const ERROR_OPEN_EXE: &str = "Failed to open generated executable";
const ERROR_OPEN_OUTPUT: &str = "Failed to open output path";
const ERROR_WRITE_EXE: &str = "Failed to copy generated executable";

#[derive(PartialEq)]
pub enum BuildType {
  Lib,
  Exe,
}

pub struct BuildContext {
  pub build_type: BuildType,
  pub target_path: PathBuf,
  pub runtime_path: PathBuf,
  pub input: PathBuf,
  pub output: PathBuf,
  pub dependencies: Vec<PathBuf>,
}

pub struct BuildArtifact {
  pub bmi: BinaryModuleInterface,
  pub code: String,
}

impl BuildContext {
  pub fn extract_dependencies(&self) -> Result<Vec<BinaryModuleInterface>> {
    let mut res = Vec::new();

    create_dir_all(&self.target_path).context(ERROR_TARGET_DIR)?;

    for dependency in self.dependencies.iter() {
      let depfile = File::open(dependency).context(ERROR_DEP_ARCHIVE)?;
      let mut depar = Archive::new(depfile);

      while let Some(entry) = depar.next_entry() {
        let mut entry = entry.context(ERROR_DEP_ARCHIVE)?;
        let filename = str::from_utf8(entry.header().identifier()).context(ERROR_DEP_ARCHIVE)?;
        let filepath = self.target_path.join(filename);
        let mut file = File::create(&filepath).context(ERROR_DEP_ARCHIVE)?;
        copy(&mut entry, &mut file).context(ERROR_DEP_ARCHIVE)?;

        if filepath.extension() == Some("bmi".as_ref()) {
          let file = File::open(&filepath).context(ERROR_DEP_ARCHIVE)?;
          let bmi = serde_json::from_reader(file).context(ERROR_DEP_ARCHIVE)?;
          res.push(bmi);
        }
      }
    }

    Ok(res)
  }

  pub fn build(&self, _deps: &[BinaryModuleInterface]) -> Result<BuildArtifact> {
    let src_file = &SourceFile::from_file(&self.input).context(ERROR_BUILD_ARTIFACT)?;

    let ast: AST<SourceLocation> = src_file.try_into()
      .map_err(|e: SyntaxError| e.to_owned())
      .context(ERROR_BUILD_ARTIFACT)?;

    let res = match self.build_type {
      BuildType::Lib => llback::compile_lib(ast),
      BuildType::Exe => llback::compile_exe(ast),
    };

    let (bmi, code) = res
      .map_err(|e: llback::prelude::CompilationError| e.to_owned())
      .context(ERROR_BUILD_ARTIFACT)?;

    Ok(BuildArtifact { bmi, code })
  }

  pub fn write(
    &self,
    artifact: BuildArtifact,
    deps: &[BinaryModuleInterface],
  ) -> Result<()> {
    let crate_path = self.target_path.join(&artifact.bmi.crate_name);
    create_dir_all(&crate_path).context(ERROR_TARGET_DIR)?;

    // Create crate's Cargo.toml
    let mut manifest = CargoManifest {
      package: CargoPackage {
        name: artifact.bmi.crate_name.clone(),
        version: "0.1.0".to_string(),
        edition: "2021".to_string(),
      },
      dependencies: HashMap::new(),
      bin: Vec::new(),
      workspace: CargoWorkspace {},
    };

    for dep in deps.iter() {
      manifest.dependencies.insert(
        dep.crate_name.clone(),
        CargoDependency::Local {
          package: dep.crate_name.clone(),
          path: self.target_path.join(&dep.crate_name),
        }
      );
    }

    manifest.dependencies.insert(
      "llruntime".to_string(),
      CargoDependency::Local {
        package: "letlang-runtime".to_string(),
        path: self.runtime_path.clone(),
      }
    );

    if self.build_type == BuildType::Exe {
      manifest.bin.push(CargoBin {
        name: {
          self.output.file_name()
            .expect("output should have a filename")
            .to_string_lossy()
            .to_string()
        },
        path: PathBuf::from("src/main.rs"),
      });
    }

    {
      let manifest_path = crate_path.join("Cargo.toml");
      let mut manifest_file = File::create(&manifest_path).context(ERROR_WRITE_MANIFEST)?;
      let manifest_data = toml::to_string(&manifest).context(ERROR_WRITE_MANIFEST)?;
      manifest_file.write_all(manifest_data.as_bytes()).context(ERROR_WRITE_MANIFEST)?;
    }

    // Write code to source file
    let src_path = crate_path.join("src");
    create_dir_all(&src_path).context(ERROR_WRITE_SOURCE)?;

    let source_path = match self.build_type {
      BuildType::Lib => src_path.join("lib.rs"),
      BuildType::Exe => src_path.join("main.rs"),
    };

    {
      let mut source_file = File::create(&source_path).context(ERROR_WRITE_SOURCE)?;
      source_file.write_all(artifact.code.as_bytes()).context(ERROR_WRITE_SOURCE)?;
    }

    // Format source code
    Command::new("rustfmt")
      .arg("--edition=2021")
      .arg(&source_path)
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .status().context(ERROR_WRITE_SOURCE)?
      .exit_ok().context(ERROR_WRITE_SOURCE)?;

    // Call cargo build
    Command::new("cargo")
      .arg("build")
      .arg("--release")
      .current_dir(&crate_path)
      .status().context(ERROR_BUILD_ARTIFACT)?
      .exit_ok().context(ERROR_BUILD_ARTIFACT)?;

    match self.build_type {
      BuildType::Lib => {
        todo!("write lib")
      },
      BuildType::Exe => {
        let mut src_file = {
          let exe_dir = crate_path.join("target").join("release");
          let exe_name = self.output.file_name()
            .expect("output should have a filename");
          let exe_path = exe_dir.join(exe_name).with_extension(std::env::consts::EXE_EXTENSION);

          File::open(&exe_path).context(ERROR_OPEN_EXE)?
        };
        let mut tgt_file = {
          let exe_path = self.output.with_extension(std::env::consts::EXE_EXTENSION);
          File::create(&exe_path).context(ERROR_OPEN_OUTPUT)?
        };

        copy(&mut src_file, &mut tgt_file).context(ERROR_WRITE_EXE)?;
      },
    }

    Ok(())
  }
}
