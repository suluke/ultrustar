# Development Task Automation
This directory contains tools that integrate with cargo (via aliases in `.cargo/config`) that automate certain development tasks.

## wasm
This utility runs all the required steps to compile the project for the web.
It also brings a http server which can be started by running `cargo wasm serve`.
