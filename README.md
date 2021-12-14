# Helipad [![CI](https://github.com/joseluisq/helipad/actions/workflows/devel.yml/badge.svg)](https://github.com/joseluisq/helipad/actions/workflows/devel.yml)

> A WIP lightweight and fast CI/CD Server written in Rust.

**NOTE:** This is a very early **proof of concept** CI/CD Server project yet so any ideas or contributions are welcome.

## TODO

- [x] Host Commands Executor (HCE)
- [x] Pipelines, steps and scripts support via an individual pipeline file. E.g [`pipeline.toml`](.pipelines/development.toml)
- [x] Pipelines, steps and scripts support via multiple pipeline files. E.g [`.pipelines/`](.pipelines/)
- [x] Environment variables support for steps
- [ ] Git source control integration (clone)
- [ ] HTTP Server
- [ ] Docker Commands Executor (DCE)
- [ ] Parallel Steps
- [ ] Parallel Pipelines
- [ ] Server REST API
- [ ] HTTP Client
- [ ] ???

## Usage

```sh
# After a build:
$ helipad -c ./.pipelines

# or via Cargo:
$ cargo run -- -c ./.pipelines
```

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in current work by you, as defined in the Apache-2.0 license, shall be dual licensed as described below, without any additional terms or conditions.

Feel free to send some [Pull request](https://github.com/joseluisq/helipad/pulls) or file an [issue](https://github.com/joseluisq/helipad/issues).

## License

This work is primarily distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

© 2021-present [Jose Quintana](https://git.io/joseluisq)
