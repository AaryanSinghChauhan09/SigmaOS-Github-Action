# SigmaOS GitHub Action Registry

This repository contains two related pieces:

- `src/core.py` and `bin/sigma-github-action`: a Python CLI for creating,
  validating, indexing, and registering action manifests.
- `src/lib.rs`: a small Rust component identity used by the SigmaOS component
  catalog.

The Python registry CLI is the functional tool; the Rust library is an
identity scaffold and does not execute GitHub Actions workflows.

## Requirements

- Python 3.11 or newer
- Rust and Cargo for the component library

## Use the registry CLI

```sh
bin/sigma-github-action --help
bin/sigma-github-action manifest --help
```

Available operations include `register`, `manifest`, `index`, and
`validate-manifest`. Use the CLI help for the exact arguments and manifest
schema.

## Build and checks

```sh
make build
make test
cargo check --lib
```

The GitHub Actions workflow compiles the Python sources and runs the Python
unit suite on pushes and pull requests.

## License

GPL-3.0-only. See [`LICENSE`](LICENSE).
