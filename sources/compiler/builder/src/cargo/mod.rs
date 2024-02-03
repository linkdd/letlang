use std::collections::HashMap;
use std::path::PathBuf;
use serde::Serialize;

#[derive(Serialize)]
pub struct CargoManifest {
  pub package: CargoPackage,
  pub dependencies: HashMap<String, CargoDependency>,
  pub bin: Vec<CargoBin>,
  pub workspace: CargoWorkspace,
}

#[derive(Serialize)]
pub struct CargoWorkspace {}

#[derive(Serialize)]
pub struct CargoPackage {
  pub name: String,
  pub version: String,
  pub edition: String,
}

#[derive(Serialize)]
pub struct CargoBin {
  pub name: String,
  pub path: PathBuf,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum CargoDependency {
  Local {
    package: String,
    path: PathBuf,
  },
  Remote {
    package: String,
    version: String,
  },
}
