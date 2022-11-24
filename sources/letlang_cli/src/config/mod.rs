use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct ProjectConfig {
  pub package: PackageConfig,
  pub executable: ExecutableConfig,
  pub toolchain: ToolchainConfig,
  pub dependencies: HashMap<String, DependencyConfig>,
}

#[derive(Deserialize, Debug)]
pub struct PackageConfig {
  pub name: String,
  pub version: String,
  pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct ExecutableConfig {
  pub bin: String,
  pub module: String,
}

#[derive(Deserialize, Debug)]
pub struct ToolchainConfig {
  pub letlang: String,
  pub rust: String,
}

#[derive(Deserialize, Debug)]
pub struct DependencyConfig {
  pub path: PathBuf,
}


impl ProjectConfig {
  pub fn load<P>(filename: P) -> Result<ProjectConfig, Box<dyn std::error::Error>>
    where P: AsRef<std::path::Path>
  {
    let content = std::fs::read_to_string(filename)?;
    let config: ProjectConfig = toml::from_str(&content)?;

    Ok(config)
  }
}
