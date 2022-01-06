# Development Task Automation
This directory contains tools that integrate with cargo (via aliases in `.cargo/config`) that automate certain development tasks.

## checkit
This utility is basically our test script for CI.
It does much more than just building and running tests, however.
Among others, it will format your code and run the linter for you.
Run this tool via `cargo checkit` before pushing to make sure your changes will pass CI.

## wasm
This utility runs all the required steps to compile the project for the web.
It also brings a http server which can be started by running `cargo wasm serve`.
