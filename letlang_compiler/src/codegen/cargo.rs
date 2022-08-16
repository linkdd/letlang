use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProjectConfig {
  pub workspace: CargoWorkspaceConfig,
}

#[derive(Serialize)]
pub struct UnitConfig {
  pub package: CargoPackageConfig,
  pub dependencies: HashMap<String, CargoDependencyConfig>,
}

#[derive(Serialize)]
pub struct CargoWorkspaceConfig {
  pub members: Vec<String>,
}

#[derive(Serialize)]
pub struct CargoPackageConfig {
  pub name: String,
  pub version: String,
  pub edition: String,
}

#[derive(Serialize)]
pub enum CargoDependencyConfig {
  Versionned(String),
  FileSystem { path: String },
}

impl UnitConfig {
  pub fn to_string(&self) -> String {
    let mut content = String::from("");

    content.push_str("[package]\n");
    content.push_str(&format!("name = \"{}\"\n", self.package.name.clone()));
    content.push_str(&format!("version = \"{}\"\n", self.package.version.clone()));
    content.push_str(&format!("edition = \"{}\"\n", self.package.edition.clone()));
    content.push_str("\n");
    content.push_str("[dependencies]\n");

    for (name, desc) in self.dependencies.iter() {
      match desc {
        CargoDependencyConfig::Versionned(ver) => {
          content.push_str(&format!("{} = \"{}\"\n", name, ver));
        },
        CargoDependencyConfig::FileSystem { path } => {
          content.push_str(&format!("{} = {{ path = \"{}\" }}\n", name, path));
        }
      }
    }

    content
  }
}