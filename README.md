# varlinkfmt

[![crates][crates]](https://crates.io/crates/varlinkfmt)

Formatter for [Varlink](https://varlink.org).
Using [tree-sitter-varlink](https://github.com/bachorp/tree-sitter-varlink) and [Topiary](https://topiary.tweag.io/).

[crates]: https://img.shields.io/crates/v/varlinkfmt?logo=rust

## Installation

```sh
cargo install varlinkfmt
```

## Usage

```sh
cat x.varlink | varlinkfmt --indent 4 > formatted.varlink
```

```sh
varlinkfmt --inplace *.varlink
```
