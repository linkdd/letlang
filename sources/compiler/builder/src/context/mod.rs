use std::{
  path::PathBuf,
  fs::{create_dir_all, File},
  io::{copy, prelude::*},
  process::{Command, Stdio},
  str,
};

use ar::Archive;

use crate::prelude::*;
use llfront::*;
use llbmi::BinaryModuleInterface;

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

impl BuildContext {
  pub fn extract_dependencies(&self) -> Result<()> {
    create_dir_all(&self.target_path)?;

    for dependency in self.dependencies.iter() {
      let depfile = File::open(dependency)?;
      let mut depar = Archive::new(depfile);

      while let Some(entry) = depar.next_entry() {
        let mut entry = entry?;
        let filename = str::from_utf8(entry.header().identifier())?;
        let filepath = self.target_path.join(filename);
        let mut file = File::create(filepath)?;
        copy(&mut entry, &mut file)?;
      }
    }

    Ok(())
  }

  pub fn build(&self) -> Result<(String, BinaryModuleInterface, String)> {
    let src_file = &SourceFile::from_file(&self.input)?;

    let ast: AST<SourceLocation> = src_file.try_into()
      .map_err(|e: SyntaxError| e.to_owned())?;

    let (crate_name, bmi, code) = match self.build_type {
      BuildType::Lib => {
        llback::compile_lib(ast).map_err(
          |e: llback::prelude::CompilationError| e.to_owned()
        )?
      },
      BuildType::Exe => {
        llback::compile_exe(ast).map_err(
          |e: llback::prelude::CompilationError| e.to_owned()
        )?
      },
    };

    Ok((crate_name, bmi, code))
  }

  pub fn write(
    &self,
    crate_name: String,
    bmi: BinaryModuleInterface,
    code: String,
  ) -> Result<()> {
    match self.build_type {
      BuildType::Lib => {
        todo!("build lib");
      },
      BuildType::Exe => {
        let rustc = Command::new("rustc")
          //.arg("--error-format=json")
          .arg("--edition=2021")
          .args(["-L", self.target_path.display().to_string().as_str()])
          .args(["-L", self.runtime_path.display().to_string().as_str()])
          .args(["--crate-type", "bin"])
          .args(["--crate-name", &crate_name])
          .args(["-l", "llruntime"])
          .args(["-o", "-"])
          .arg("-")
          .stdin(Stdio::piped())
          .stdout(Stdio::piped())
          //.stderr(Stdio::piped())
          .spawn()?;

        rustc.stdin.unwrap().write_all(code.as_bytes())?;
        let mut exe: Vec<u8> = vec![];
        rustc.stdout.unwrap().read_to_end(&mut exe)?;
        todo!("build exe");
      }
    }
  }
}
