use anyhow::Result;
use argh::FromArgs;
use std::path::PathBuf;
use wasm_bindgen_cli_support::{Bindgen, EncodeInto};
use xshell::{cmd, pushd};

#[derive(FromArgs)]
/// Utility to build the wasm target
struct Cli {
    /// build in release mode
    #[argh(switch)]
    release: bool,

    #[argh(subcommand)]
    subcommand: Option<CliSubcommands>,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum CliSubcommands {
    Serve(CliServe),
}

#[derive(FromArgs)]
/// Start a web server to serve the project
#[argh(subcommand, name = "serve")]
struct CliServe {
    /// port used for serving
    #[argh(option, short = 'p')]
    port: Option<usize>,

    /// disable auto-compiling
    #[argh(switch)]
    only: bool,
}

fn project_root() -> Result<PathBuf> {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if !dir.pop() {
        return Err(anyhow::Error::msg("Expected parent directory"));
    }
    if !dir.pop() {
        return Err(anyhow::Error::msg("Expected parent directory"));
    }
    Ok(dir)
}

fn target_dir() -> Result<PathBuf> {
    let mut dir = project_root()?;
    dir.push("target");
    dir.push("wasm32-unknown-unknown");
    dir.push("debug");
    Ok(dir)
}

fn build(args: &Cli) -> Result<()> {
    let rootdir = project_root()?;
    let _ = pushd(rootdir)?;
    cmd!("cargo build -p ultrustar --target wasm32-unknown-unknown")
        .env_remove("CARGO_MANIFEST_DIR")
        .run()?;
    let mut wasm_path = target_dir()?;
    wasm_path.push("ultrastar_core.wasm");
    let mut builder = Bindgen::new();

    builder
        .input_path(wasm_path)
        .web(true)?
        .debug(!args.release)
        .keep_debug(!args.release)
        .out_name("ultrustar")
        .encode_into(EncodeInto::Always);
    builder.generate(target_dir()?)
}

fn serve(args: &Cli) -> Result<()> {
    use actix_files::Files;
    use actix_web::{dev::Service, rt, App, HttpServer};

    let CliSubcommands::Serve(serve_opts) = args.subcommand.as_ref().unwrap();
    let port = serve_opts.port.unwrap_or(3000);
    println!("Serving on http://localhost:{}/index.html", port);

    let mut sys = rt::System::new("ultrustar-dev");
    if serve_opts.only {
        let srv = HttpServer::new(|| {
            let rootdir = project_root().unwrap();
            App::new().service(Files::new("/", rootdir))
        })
        .bind(format!("127.0.0.1:{}", port))?
        .run();
        sys.block_on(srv).map_err(anyhow::Error::from)
    } else {
        let srv = HttpServer::new(|| {
            let rootdir = project_root().unwrap();
            App::new()
                .wrap_fn(|req, srv| {
                    if req.path().ends_with(".wasm") {
                        let args: Cli = argh::from_env();
                        build(&args).unwrap();
                    }
                    srv.call(req)
                })
                .service(Files::new("/", rootdir))
        })
        .bind(format!("127.0.0.1:{}", port))?
        .run();
        sys.block_on(srv).map_err(anyhow::Error::from)
    }
}

fn main() -> Result<()> {
    let args: Cli = argh::from_env();
    if let Some(subcommand) = &args.subcommand {
        match subcommand {
            CliSubcommands::Serve(_) => serve(&args),
        }
    } else {
        build(&args)
    }
}
