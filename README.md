# TF Demo Parser

![Build Status](https://github.com/demostf/parser/workflows/CI/badge.svg)

Parsing of tf2 demo files

## Building

This project is build using rust and requires `cargo` and friends, see [the rust website](https://www.rust-lang.org/)
for how to get started.

Once rust is setup building is as simple as

```bash
cargo build --release
```

which will place the binary at `target/release/parse_demo`

## Usage

Basic usage is as simple as `parse_demo demofile.dem` which will output a "summary" of the demo file in JSON format.