funst
=====

[![funst](https://img.shields.io/crates/v/funst.svg)](https://crates.io/crates/funst)
[![Documentation](https://docs.rs/funst/badge.svg)](https://docs.rs/funst)
[![Actions Status](https://github.com/sile/funst/workflows/CI/badge.svg)](https://github.com/sile/funst/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

`funst` is a tiny command-line tool to calculate fundamental statistics of numbers given via the standard input.

```console
$ seq 1 100 | funst
{
  "count": 100,
  "mean": 50.5,
  "stddev": 28.86607004772212,
  "min": 1.0,
  "median": 50.5,
  "max": 100.0
}
```

Installation
------------

### Precompiled binaries

Precompiled binaries for Linux are available in the [releases] page.

```console
$ curl -L https://github.com/sile/funst/releases/download/${VERSION}/funst-${VERSION}.linux-amd64 -o funst
$ chmod +x funst
$ ./funst -h
```

[releases]: https://github.com/sile/funst/releases

### Using Cargo

If you have already installed [Cargo][cargo], you can install `funst` by executing the following command:

```console
$ cargo install funst
```

[cargo]: https://doc.rust-lang.org/cargo/
