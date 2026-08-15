use anyhow::{bail, Context, Result};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};
fn main() -> Result<()> {
    let mut a = env::args().skip(1);
    if a.next().as_deref() != Some("demo") {
        bail!("usage: axiom-node demo --bin-dir DIR --work-dir DIR");
    }
    let bin = PathBuf::from(arg(&mut a, "--bin-dir")?);
    let work = PathBuf::from(arg(&mut a, "--work-dir")?);
    fs::create_dir_all(&work)?;
    let ax = work.join("abs.ax");
    let aix = work.join("abs.aix");
    let axp = work.join("abs.axp");
    let proof = work.join("abs.axproof");
    let dag = work.join("proof.axdag");
    fs::write(&ax,"axiom 0.1\nmodule abs\ninput x i64\noutput result i64\ndomain x -16 16\nrequires true\nensures result >= 0\nensures result == x || result == -x\nobjective instructions min\n")?;
    run(&bin, "axiom-spec", &["compile", s(&ax), "--out", s(&aix)])?;
    run(
        &bin,
        "axiom-synth",
        &["synth", s(&aix), "--out", s(&axp), "--max-depth", "3"],
    )?;
    run(
        &bin,
        "axiom-verifier",
        &["verify", s(&aix), s(&axp), "--proof", s(&proof)],
    )?;
    run(
        &bin,
        "axiom-proof",
        &["append", s(&dag), s(&proof), "--label", "abs-v1"],
    )?;
    run(
        &bin,
        "axiom-runtime",
        &[
            "run",
            "--spec",
            s(&aix),
            "--program",
            s(&axp),
            "--proof",
            s(&proof),
            "--x",
            "-13",
        ],
    )?;
    println!("pipeline complete: {}", work.display());
    Ok(())
}
fn arg(a: &mut impl Iterator<Item = String>, n: &str) -> Result<String> {
    let f = a.next().context("missing flag")?;
    if f != n {
        bail!("expected {n}");
    }
    a.next().context("missing value")
}
fn s(p: &Path) -> &str {
    p.to_str().expect("UTF-8 path")
}
fn run(bin: &Path, name: &str, args: &[&str]) -> Result<()> {
    let exe = if cfg!(windows) {
        bin.join(format!("{name}.exe"))
    } else {
        bin.join(name)
    };
    println!("+ {} {}", exe.display(), args.join(" "));
    let status = Command::new(&exe)
        .args(args)
        .status()
        .with_context(|| format!("failed to run {}", exe.display()))?;
    if !status.success() {
        bail!("{name} failed with {status}");
    }
    Ok(())
}
