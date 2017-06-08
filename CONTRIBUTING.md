# Contributing to stdx

As a contributor to stdx, the main thing to be aware of when adding or
modifying examples is the conventions around how the examples are
organized. There are a number of elements that need to be kept in
sync.

The main product of stdx is its README.md file, which contains an
index of crates, their examples, links to their documentation, and
links to the example as a source file. The Cargo.toml file must be
maintained as well.

Crates in stdx are categorized as

- _primary_ - these are the crates of stdx. They are represented in
  the Cargo.toml file and the index.
- _supplemental_ - these are crates users are likely to want to use with
  a stdx crate. They are listed in the crate description as
  "supplemental", and may or may not be part of index. They are
  included of the Cargo.toml file.
- _alternatives_ - These are not part of stdx but are notable
  alternatives that fulfill the same functions as crates in stdx.

Everywhere crates appear they are listed in alphabetical order.  This
includes Cargo.toml, the index, and the link definitions in the
markdown.

Crates are often described in the form `cratename = "version"`. This
is so it can be directly copied into the user's Cargo.toml. But it
results in a bunch of duplication for stdx maintenance.

The bits that need to be maintained are:

- The version numbers in Cargo.toml
- The version numbers in the index
- The version numbers in index in README.md
- The version numbers in the crate headers in README.md
- The version numbers in the link definitions at the end of README.md

All links are defined at thet end of README.md

Every crate has an example, and that example is duplicated in both
README.md and in the `examples/` folder. So when updating an example
make sure to do it in both places.

Test that the examples at least build with:

```
cargo build --examples
```

Note that `--examples` was added only in Rust 1.20.
