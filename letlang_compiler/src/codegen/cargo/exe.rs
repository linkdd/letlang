pub use super::Generator;

use serde::Serialize;
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::error::Error;


#[derive(Serialize)]
struct CargoConfig {
  package: PackageConfig,
  dependencies: HashMap<String, DependencyConfig>,
}

#[derive(Serialize)]
struct PackageConfig {
  name: String,
  version: String,
  edition: String,
}

#[derive(Serialize)]
struct DependencyConfig {
  path: Option<String>,
  version: Option<String>,
  features: Option<Vec<String>>,
}


impl<'compiler> Generator<'compiler> {
  pub fn gen_exe_crate<P: AsRef<Path>>(
    &mut self,
    target_dir: P,
  ) -> Result<(), Box<dyn Error>> {
    let main_crate = format!(
      "lldep_{}",
      self.target.main_module.replace("::", "_")
    );

    let mut folder_path = PathBuf::new();
    folder_path.push(target_dir);
    folder_path.push("executable");
    std::fs::create_dir_all(&folder_path)?;

    let mut crate_cargo_path = PathBuf::new();
    crate_cargo_path.push(&folder_path);
    crate_cargo_path.push("Cargo.toml");

    let mut crate_deps: HashMap<String, DependencyConfig> = HashMap::new();

    let runtime_dep = match std::env::var("LETLANG_RUNTIME_PATH") {
      Err(_) => DependencyConfig {
        path: None,
        version: Some(self.toolchain.runtime_version.clone()),
        features: None,
      },
      Ok(path) => DependencyConfig {
        path: Some(path),
        version: None,
        features: None,
      },
    };

    crate_deps.insert("llcore_runtime".to_string(), runtime_dep);
    crate_deps.insert(
      main_crate.clone(),
      DependencyConfig {
        path: Some(format!("../modules/{}", main_crate.clone())),
        version: None,
        features: None,
      }
    );

    let crate_cargo_cfg = CargoConfig {
      package: PackageConfig {
        name: self.target.binary_name.clone(),
        version: self.target.package_version.clone(),
        edition: self.toolchain.rust_edition.clone(),
      },
      dependencies: crate_deps,
    };
    std::fs::write(crate_cargo_path, toml::to_string(&crate_cargo_cfg)?)?;

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
}
