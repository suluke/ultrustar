use anyhow::{anyhow, Result};
use argh::FromArgs;
use lazy_static::lazy_static;
use xshell::cmd;

#[derive(FromArgs)]
/// Project-specific checks
struct Cli {}

trait Check: Sync {
    fn check(&self) -> Result<()>;
}

struct CheckGitClean;
impl Check for CheckGitClean {
    fn check(&self) -> Result<()> {
        let stdout = cmd!("git status --porcelain=v1").read()?;
        if stdout.is_empty() {
            Ok(())
        } else {
            Err(anyhow!(format!("Git repo not clean:\n{}", stdout)))
        }
    }
}

struct CheckFmt;
impl Check for CheckFmt {
    fn check(&self) -> Result<()> {
        cmd!("cargo fmt  --all").run().map_err(anyhow::Error::from)?;
        CheckGitClean.check()
    }
}

struct CheckClippy;
impl Check for CheckClippy {
    fn check(&self) -> Result<()> {
        cmd!("cargo clippy").run().map_err(anyhow::Error::from)
    }
}

type CheckRegistry = std::collections::BTreeMap<&'static str, Box<dyn Check>>;
lazy_static! {
    static ref ALL_CHECKS: CheckRegistry = {
        let mut all_checks = CheckRegistry::new();
        all_checks.insert("clean-git", Box::new(CheckGitClean));
        all_checks.insert("fmt", Box::new(CheckGitClean));
        all_checks.insert("clippy", Box::new(CheckClippy));
        all_checks
    };
}

fn run_checks() -> Result<()> {
    for (_, check) in ALL_CHECKS.iter() {
        check.check()?;
    }
    Ok(())
}

fn main() {
    if let Err(err) = run_checks() {
        eprintln!("{}", err);
        std::process::exit(69);
    }
}
