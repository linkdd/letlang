use letlang_cli::{
  prelude::*,
  args::Args,
};

use clap::Parser;

fn main() {
  let args = Args::parse();

  if let Err(CliError { message }) = build(args) {
    eprintln!("{message}");
    std::process::exit(1);
  }
}

fn build(args: Args) -> Result<()> {
  let ctx = args.to_build_context()?;
  ctx.extract_dependencies()?;
  let (crate_name, bmi, code) = ctx.build()?;
  ctx.write(crate_name, bmi, code)?;
  Ok(())
}
