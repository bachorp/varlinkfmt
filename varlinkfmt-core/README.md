# varlinkfmt-core

[![crates][crates]](https://crates.io/crates/varlinkfmt-core)

Formatter for [Varlink](https://varlink.org).
Built on [tree-sitter-varlink](https://github.com/bachorp/tree-sitter-varlink) and [Topiary](https://topiary.tweag.io/).

See also [varlinkfmt](https://crates.io/crates/varlinkfmt).

[crates]: https://img.shields.io/crates/v/varlinkfmt-core?logo=rust

## Example

```rust
use varlinkfmt_core::{Indent, Options, formatter_str, mk_language, topiary_core::FormatterResult};

fn run() -> FormatterResult<()> {
    let input = "\
        interface org.example.foo \
        type Bar(baz:\nbool) \
        method M  (foo: string) ->(bar: Bar )";

    let mut out = Vec::new();
    let language = mk_language(Indent::Spaces(4));
    let opts = Options::default();

    // Also available: `formatter`, `formatter_tree`
    formatter_str(input, &mut out, &language, opts)?;
    print!("{}", String::from_utf8(out).unwrap());
    Ok(())
}
```
