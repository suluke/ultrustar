use anyhow::{anyhow, Result};
use argh::FromArgs;
use lazy_static::lazy_static;
use xshell::cmd;

#[derive(FromArgs)]
/// Project-specific checks
struct Cli {}

trait Check: Sync {
    fn key(&self) -> &'static str;
    fn check(&self) -> Result<()>;
}

macro_rules! new_check {
    ($name:ident, $id:literal, $impl:block) => {
        struct $name;
        impl Check for $name {
            fn key(&self) -> &'static str {
                $id
            }
            fn check(&self) -> Result<()> {
                $impl
            }
        }
    };
}

new_check!(CheckBuild, "build", {
    cmd!("cargo build --verbose")
        .run()
        .map_err(anyhow::Error::from)
});
new_check!(CheckTests, "test", {
    cmd!("cargo test --verbose")
        .run()
        .map_err(anyhow::Error::from)
});
new_check!(CheckWasm, "test", {
    cmd!("cargo wasm --release")
        .run()
        .map_err(anyhow::Error::from)
});
new_check!(CheckGitClean, "clean-git", {
    let stdout = cmd!("git status --porcelain=v1").read()?;
    if stdout.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(format!("Git repo not clean:\n{}", stdout)))
    }
});
new_check!(CheckFmt, "fmt", {
    cmd!("cargo fmt --all").run().map_err(anyhow::Error::from)?;
    CheckGitClean.check()
});
new_check!(CheckClippy, "clippy", {
    cmd!("cargo clippy").run().map_err(anyhow::Error::from)
});

struct CheckRegistry {
    checks: Vec<Box<dyn Check>>,
}
impl CheckRegistry {
    fn new() -> Self {
        let checks: Vec<Box<dyn Check>> = vec![
            Box::new(CheckBuild),
            Box::new(CheckTests),
            Box::new(CheckWasm),
            Box::new(CheckGitClean),
            // MUST run after build because rustfmt will complain if generated source files
            // (bindings) are missing.
            Box::new(CheckFmt),
            Box::new(CheckClippy),
        ];
        Self { checks }
    }
    fn iter(&self) -> std::slice::Iter<Box<dyn Check>> {
        self.checks.iter()
    }
}

lazy_static! {
    static ref ALL_CHECKS: CheckRegistry = CheckRegistry::new();
}

fn run_checks() -> Result<()> {
    for check in ALL_CHECKS.iter() {
        println!("Running check: {}", check.key());
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
