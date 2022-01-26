#![deny(
    unsafe_code,
    unused_imports,
    clippy::all,
    clippy::complexity,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

use anyhow::{anyhow, Result};
use argh::FromArgs;
use xshell::cmd;

#[derive(FromArgs)]
/// Project-specific checks
struct Cli {
    /// run the specified check
    #[argh(option, short = 'c')]
    check: Option<String>,

    /// check verbosity
    #[argh(switch, short = 'v')]
    verbose: bool,
}

trait Check: Sync {
    fn key(&self) -> &'static str;
    fn check(&self, opts: &Cli) -> Result<()>;
}

macro_rules! new_check {
    ($name:ident, $id:literal, $opts:ident, $impl:block) => {
        struct $name;
        impl Check for $name {
            fn key(&self) -> &'static str {
                $id
            }
            fn check(&self, $opts: &Cli) -> Result<()> {
                $impl
            }
        }
    };
}

new_check!(CheckBuild, "build", opts, {
    if opts.verbose {
        cmd!("cargo build --verbose")
    } else {
        cmd!("cargo build")
    }
    .run()
    .map_err(anyhow::Error::from)
});
new_check!(CheckTests, "test", opts, {
    if opts.verbose {
        cmd!("cargo test --verbose")
    } else {
        cmd!("cargo test")
    }
    .run()
    .map_err(anyhow::Error::from)
});
new_check!(CheckWasm, "test", _opts, {
    cmd!("cargo wasm --release")
        .run()
        .map_err(anyhow::Error::from)
});
new_check!(CheckGitClean, "clean-git", _opts, {
    let stdout = cmd!("git status -uno --porcelain=v1").read()?;
    if stdout.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(format!("Git repo not clean:\n{}", stdout)))
    }
});
new_check!(CheckFmt, "fmt", opts, {
    cmd!("cargo fmt --all").run().map_err(anyhow::Error::from)?;
    CheckGitClean.check(opts)
});
new_check!(CheckClippy, "clippy", _opts, {
    cmd!("cargo clippy").run().map_err(anyhow::Error::from)
});
new_check!(CheckDocs, "doc", _opts, {
    cmd!("cargo doc --workspace --no-deps")
        .run()
        .map_err(anyhow::Error::from)
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
            Box::new(CheckDocs),
        ];
        Self { checks }
    }
    fn iter(&self) -> std::slice::Iter<Box<dyn Check>> {
        self.checks.iter()
    }
}

fn run_checks() -> Result<()> {
    let options: Cli = argh::from_env();
    let checks = CheckRegistry::new();
    if let Some(check) = &options.check {
        // run specified check (if found)
        if let Some(check) = checks.iter().find(|&c| c.key() == check) {
            println!("Running check: {}", check.key());
            check.check(&options)?;
        } else {
            return Err(anyhow!("Unknown check: {}", check));
        }
    } else {
        // run all
        for check in checks.iter() {
            println!("Running check: {}", check.key());
            check.check(&options)?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = run_checks() {
        eprintln!("{}", err);
        std::process::exit(69);
    }
}
