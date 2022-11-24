pub use super::Generator;

use serde::Serialize;
use std::path::{PathBuf, Path};
use std::error::Error;


#[derive(Serialize)]
struct CargoConfig {
  workspace: WorkspaceConfig,
}

#[derive(Serialize)]
struct WorkspaceConfig {
  members: Vec<String>
}

impl<'compiler> Generator<'compiler> {
  pub fn gen_workspace<P: AsRef<Path>>(
    &mut self,
    target_dir: P,
    members: Vec<String>,
  ) -> Result<(), Box<dyn Error>> {
    let config = CargoConfig {
      workspace: WorkspaceConfig { members }
    };

    let mut cargo_path = PathBuf::new();
    cargo_path.push(target_dir);
    cargo_path.push("Cargo.toml");

    std::fs::write(cargo_path, toml::to_string(&config)?)?;

    Ok(())
  }
}
