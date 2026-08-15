use anyhow::{bail, Context, Result};
use std::{env, fs, path::PathBuf};

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    if args.next().as_deref() != Some("init") {
        bail!("usage: axiom-node init --workspace DIR");
    }
    if args.next().as_deref() != Some("--workspace") {
        bail!("expected --workspace");
    }
    let workspace = PathBuf::from(args.next().context("missing workspace")?);
    for dir in [
        "specs",
        "candidates",
        "counterexamples",
        "proofs",
        "dag",
        "runtime",
    ] {
        fs::create_dir_all(workspace.join(dir))?;
    }
    fs::write(
        workspace.join("node.manifest"),
        "AXIOM-NODE/1\npolicy=proof-before-execute\nevolution=counterexample-guided\n",
    )?;
    println!("initialized {}", workspace.display());
    Ok(())
}
