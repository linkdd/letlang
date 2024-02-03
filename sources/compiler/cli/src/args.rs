use std::{
  ffi::OsString,
  path::{absolute, PathBuf},
  fs::canonicalize,
};

use clap::{Parser, ValueEnum, ValueHint};
use anyhow::{bail, Result};

use llbuild::context::{BuildType, BuildContext};

trait ToAbsolute {
  fn as_absolute(&self) -> Self;
  fn as_canonical(&self) -> Self;
}

impl ToAbsolute for PathBuf {
  fn as_absolute(&self) -> PathBuf {
    match absolute(&self) {
      Ok(path) => path,
      Err(err) => panic!("ERROR: Could not absolutize path: {}\n{:?}", self.display(), err),
    }
  }

  fn as_canonical(&self) -> PathBuf {
    match canonicalize(&self) {
      Ok(path) => path,
      Err(err) => panic!("ERROR: Could not canonicalize path: {}\n{:?}", self.display(), err),
    }
  }
}

#[derive(ValueEnum, Debug, Clone)]
pub enum UnitType {
  Lib,
  Exe,
}

impl From<UnitType> for BuildType {
  fn from(value: UnitType) -> Self {
    match value {
      UnitType::Lib => BuildType::Lib,
      UnitType::Exe => BuildType::Exe,
    }
  }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  /// Type of build ("lib" to build a static library, "exe" to build an executable)
  #[arg(short = 't', long = "type", value_enum, default_value = "lib")]
  unit_type: UnitType,

  /// Path to Letlang source code
  #[arg(value_name = "FILE", value_hint = ValueHint::FilePath)]
  input: PathBuf,

  /// Path to build artifact
  #[arg(short, value_name = "FILE")]
  output: Option<PathBuf>,

  /// Path to store extracted dependencies
  #[arg(short = 'b', value_name = "DIR", value_hint = ValueHint::DirPath)]
  build_dir: Option<PathBuf>,

  /// Path to the Letlang runtime Cargo project
  #[arg(long = "rpath", value_name = "DIR", value_hint = ValueHint::DirPath)]
  runtime_path: PathBuf,

  /// Add folder to library paths, this is where the compiler will look for dependencies
  #[arg(short = 'L', value_name = "DIR", value_hint = ValueHint::DirPath)]
  library_path: Vec<PathBuf>,

  /// Add a dependency
  #[arg(short = 'l', value_name = "LIBNAME")]
  link_lib: Vec<String>,
}

impl Args {
  pub fn to_build_context(self) -> Result<BuildContext> {
    let input = self.input.as_canonical();
    let output = self.output.unwrap_or_else(|| {
      let unit_name = input.file_name().expect("unable to get input filename");

      match self.unit_type {
        UnitType::Lib => {
          let mut name = OsString::from("lib");
          name.push(unit_name);
          PathBuf::from(name).with_extension("lla")
        },
        UnitType::Exe => {
          PathBuf::from(unit_name).with_extension("")
        },
      }
    }).as_absolute();

    let build_dir = self.build_dir
      .unwrap_or_else(|| PathBuf::from(".lltarget"))
      .as_absolute();

    let runtime_path = self.runtime_path.as_canonical();

    let library_paths: Vec<PathBuf> = self.library_path
      .into_iter()
      .map(|p| p.as_canonical())
      .collect();

    let dependency_names = self.link_lib;

    if !input.exists() {
      bail!("No such file or directory: {}", input.display());
    }
    else if !input.is_file() {
      bail!("Expected a file at: {}", input.display());
    }

    for library_path in library_paths.iter() {
      if !library_path.exists() {
        bail!("No such file or directory: {}", library_path.display());
      }
      else if !library_path.is_dir() {
        bail!("Expected a directory at: {}", library_path.display());
      }
    }

    let mut dependency_paths = Vec::with_capacity(dependency_names.len());

    for dependency_name in dependency_names.iter() {
      let libname = format!("lib{dependency_name}.lla");

      let deppaths: Vec<PathBuf> = library_paths
        .iter()
        .map(|libdir| libdir.join(&libname))
        .filter(|p| p.is_file())
        .collect();

      match deppaths.as_slice() {
        [] => {
          bail!("Could not find {libname} in any library path.");
        },
        [deppath] => {
          dependency_paths.push(deppath.clone())
        },
        _ => {
          bail!("Multiple {libname} were found in library paths.");
        },
      };
    }

    Ok(BuildContext {
      build_type: self.unit_type.into(),
      target_path: build_dir,
      runtime_path,
      input,
      output,
      dependencies: dependency_paths,
    })
  }
}
