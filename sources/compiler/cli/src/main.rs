use letlang_cli::args::Args;

use clap::Parser;
use anyhow::Result;

fn main() {
  let args = Args::parse();

  if let Err(err) = build(args) {
    eprintln!("{err}");
    std::process::exit(1);
  }
}

fn build(args: Args) -> Result<()> {
  let ctx = args.to_build_context()?;
  let deps = ctx.extract_dependencies()?;
  let artifact = ctx.build(&deps)?;
  ctx.write(artifact, &deps)?;
  Ok(())
}
