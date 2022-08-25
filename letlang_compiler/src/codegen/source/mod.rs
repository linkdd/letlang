pub use super::Generator;

use std::path::Path;
use std::error::Error;
use std::process::Command;


impl<'compiler> Generator<'compiler> {
  pub fn reformat_source<P: AsRef<Path>>(
    &mut self,
    source_path: P,
  ) -> Result<(), Box<dyn Error>> {
    let status = Command::new("rustfmt")
      .arg(format!("--edition={}", self.toolchain.rust_edition))
      .arg(source_path.as_ref().to_path_buf())
      .status()?;

    if !status.success() {
      eprintln!("WARN: rustfmt command exited with non-zero code: {:?}", status.code());
    }

    Ok(())
  }
}

mod exe;
mod module;
