# toolbox image generator (tigen)

[![Rust](https://github.com/prx0/tig/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/prx0/tig/actions/workflows/rust.yml)
[![Crate.io](https://img.shields.io/badge/crates.io-tigen-orange)](https://crates.io/crates/tigen)
[![Docs](https://img.shields.io/badge/docs-tigen-success)](https://docs.rs/tigen/0.1.0/libtigen/)

Toolbox image builder (tigen) is a simple utility to create oci images to for creating [toolbox](https://github.com/containers/toolbox) sessions.
`docker` or `podman` must be installed and available in your `$PATH`.

Install tigen using cargo:
```sh
cargo install tigen
```

```sh
tigen --image ubuntu:20.04
```

tigen use docker image name to build oci images. After that, you'll be able to jump into ubuntu lts 20.02 using toolbox:

```sh
toolbox enter -d ubuntu -r 20.04
```