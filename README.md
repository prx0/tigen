# toolbox image generator (tigen)

[![Rust](https://github.com/prx0/tig/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/prx0/tig/actions/workflows/rust.yml)

Toolbox image builder (tigen) is a simple utility to create oci images to for creating [toolbox](https://github.com/containers/toolbox) sessions.
`docker` or `podman` must be installed and available in your `$PATH`.

Install tigen using cargo:
```sh
cargo install tigen
```

```sh
tigen -d ubuntu -r 20:04
```