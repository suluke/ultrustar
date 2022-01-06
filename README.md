# ultrustar

![CI](https://github.com/suluke/ultrustar/actions/workflows/ci.yml/badge.svg)

## Build Instructions

### Generic Preparations
Install Rust via the `rustup` toolchain manager.

### WASM
The "normal" way to compile rust code for the web nowadays would be to use the [`wasm-pack`](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html#build-the-project) utility.
However, I don't really like installing a separate tool globally just to build a variant of a project.
Furthermore, `wasm-pack` doesn't do much anyways if you look more closely and instead makes the build process more opaque.
I therefore decided to go the extra mile and keep all the build logic within `cargo` instead.

To get started with the wasm build you will need to install the wasm target like so:
```
rustup target install wasm32-unknown-unknown
```
Afterwards, our custom cargo subcommand `cargo wasm --release` will do the rest.

> Tip for vscode: When developing for the browser it makes sense to change the default target in `settings.json` like so: `"rust-analyzer.cargo.target": "wasm32-unknown-unknown"`

## Development Tips
* Run `cargo checkit` locally to see if our CI checks are ok, _especially_ before opening a PR or pushing to `main`
* By running `cargo wasm serve` you can quickly start a local development server to try out the `wasm` build

## Related Projects
* [UltraStar Deluxe](https://github.com/UltraStar-Deluxe/USDX)
* [UltraStar Play](https://github.com/UltraStar-Deluxe/Play)
* [Performous](https://github.com/performous/performous)
* [Vocaluxe](https://github.com/Vocaluxe/Vocaluxe)
