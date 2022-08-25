pub use super::Generator;

use letlang_ast::{Node, Unit};

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
  pub fn gen_lib_crate<P: AsRef<Path>>(
    &mut self,
    target_dir: P,
    crate_name: &String,
    unit_node: &Node<Unit>,
  ) -> Result<(), Box<dyn Error>> {
    let mut folder_path = PathBuf::new();
    folder_path.push(target_dir);
    folder_path.push("modules");
    folder_path.push(&crate_name);
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

    for dependency in unit_node.attrs.as_ref().unwrap().dependencies.iter() {
      crate_deps.insert(dependency.clone(), DependencyConfig {
        path: Some(format!("../modules/{}", dependency)),
        version: None,
        features: None,
      });
    }

    let crate_cargo_cfg = CargoConfig {
      package: PackageConfig {
        name: crate_name.clone(),
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

    let mut lib_path = PathBuf::new();
    lib_path.push(&source_dir);
    lib_path.push("lib.rs");

    let source_code = self.gen_lib_source(unit_node)?;
    std::fs::write(&lib_path, source_code)?;
    self.reformat_source(&lib_path)?;

    Ok(())
  }
}
