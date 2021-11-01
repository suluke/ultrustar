# ultrustar

## Build Instructions

### Generic Preparations
Install Rust via the `rustup` toolchain manager.

### WASM
The "normal" way to compile rust code for the web nowadays would be to use the [`wasm-pack`](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html#build-the-project) utility.
However, I'm don't really like installing a separate tool globally just to build a variant of a project.
Furthermore, `wasm-pack` doesn't do much anyways if you look more closely and instead makes the build process more opaque.
I therefore decided to go the extra mile and keep all the build logic within cargo instead.

To get started with the wasm build you will need to install the wasm target like so:
```
rustup target install wasm32-unknown-unknown
```
Afterwards, our custom cargo subcommand `cargo wasm` will do the rest.

> Tip for vscode: When developing for the browser it makes sense to change the default target in `settings.json` like so: `"rust-analyzer.cargo.target": "wasm32-unknown-unknown"`


## Related Projects
* [UltraStar Deluxe](https://github.com/UltraStar-Deluxe/USDX)
* [UltraStar Play](https://github.com/UltraStar-Deluxe/Play)
* [Performous](https://github.com/performous/performous)
* [Vocaluxe](https://github.com/Vocaluxe/Vocaluxe)
