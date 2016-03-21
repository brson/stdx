# The missing batteries of Rust

**stdx** is a project to collect the best Rust crates and validate
that they work together on the platforms that Rust supports.

[![Build Status](https://travis-ci.org/brson/stdx.svg?branch=master)](https://travis-ci.org/rust-lang/brson/stdx)

# Current crates

The current revision of **stdx** is "0.107.0", which corresponds to
the Rust stable 1.7 release.

* [`bitflags-0.5.0`](https://crates.io/crates/bitflags/0.5.0). The
  only thing this crate does is export the
  [`bitflags!`](http://doc.rust-lang.org/bitflags/bitflags/macro.bitflags!.html#example)
  macro, but it's a heckuva-useful macro. `bitflags!` produces
  typesafe bitmasks, types with named values that are efficiently
  packed together as bits to express sets of options. Official
  [rust-lang] crate.

* [`docopt-0.6.78`](https://crates.io/crates/docopt/0.6.78). The
  preferred way to parse command line options in Rustland. Originally
  created by Rust API design pioneer,
  [BurntSushi](http://github.com/burntsushi), it is used by Cargo and
  many other projects, but notably not by rustc and rustdoc, which use
  the older [`getopts`](https://crates.io/crates/getopts) crate.

* [`env_logger-0.3.2`](https://crates.io/crates/env_logger/0.3.2). In
  conjunction with the [`log`](https://crates.io/crates/log) crate,
  enables the output of logs to the console via the
  [`RUST_LOG`](http://doc.rust-lang.org/log/env_logger/index.html#enabling-logging)
  environment variable. This was a feature of the Rust language and
  runtime since ancient times, but is now implemented in an external
  library, and is still the most popular way to log information about
  what your Rust program is. Official [rust-lang] crate.

* [`itertools-0.4.10`](https://crates.io/crates/itertools/0.4.10).
  When it comes to iterators, this crate has everything *including*
  the kitchen sink (in the form of the `batching` adaptor).
  Highlights include `dedup`, `group_by`, `mend_slices`, `merge`,
  `join` and more.

* [`lazy_static-0.1.15`](https://crates.io/crates/lazy_static/0.1.15).
  Global state is one of those things Rust doesn't do so well. In
  particular there is no ['life before
  main'](https://isocpp.org/wiki/faq/ctors#static-init-order) in Rust,
  so it's not possible to write a programmatic constructor for a
  global value that will be run at startup. Instead, Rust prefers lazy
  execution for global initialization, and the
  [`lazy_static!`](http://rust-ci.org/Kimundi/lazy-static.rs/doc/lazy_static/)
  macro does just that.

* [`libc-0.2.7`](https://crates.io/crates/libc/0.2.7). If you need to
  talk to foreign code, you need this crate. It contains declarations
  for a grab bag of C types and functions that are correct for the
  variety of compilers and platforms that Rust runs on. This crate is
  a *notorious mess* design-wise, but it has endured for years as the
  foundation Rust uses to talk to the outside world. Official
  [rust-lang] crate.

* [`log-0.3.5`](https://crates.io/crates/log/0.3.5). The most common
  way to perform basic logging in Rust, with the `error!`, `warn!`,
  `info!`, and `debug!` macros, always used in conjunction with the
  [`env_logger`](https://crates.io/crates/env_logger) crate.
  Official [rust-lang] crate.

* [`num-0.1.31`](https://crates.io/crates/num/0.1.31). Big integers,
  rational numbers, complex numbers, and a 'numeric tower' of numeric
  traits. This is another rust-lang crate that has persisted through
  Rust's evolution but is not designed well enough for the standard
  library. It is though presently the most common way to access the
  functionality it provides.

* [`rand-0.3.14`](https://crates.io/crates/rand/0.3.14). Random number
  generators. The defaults are cryptographically strong. Official
  [rust-lang] crate.

* [`regex-0.1.55`](https://crates.io/crates/regex/0.1.55). Another
  [BurntSushi](http://github.com/burntsushi) joint, this a very
  performant regular expression implementation that [stomps the
  competition](http://benchmarksgame.alioth.debian.org/u64/performance.php?test=regexdna)
  in some benchmarks. Influenced by the highly-regarded
  [RE2](https://github.com/google/re2) engine, it omits backreferences
  and arbitrary lookahead in order to have predictable worst-case
  performance. Official [rust-lang] crate.

* [`rustc-serialize-0.3.18`](https://crates.io/crates/rustc-serialize/0.3.18).
  Another crate with a storied history, it was designed long ago to be
  Rust's solution for serialization, but time and better judgement
  consigned it to the Rust junkyard. Curiously, this is the only
  external crate that `rustc` has explicit knowledge of, since
  `#[derive(RustcEncodable, RustcDecodable)]` is hard-coded into the
  compiler, and at the time the `rustc-serialize` crate was demoted it
  was deemed too inconvenient to users to remove the deriving
  mode. [Serde](https://github.com/serde-rs/serde) is a more modern,
  and faster alternative. Official [rust-lang] crate.

* [`semver-0.2.3`](https://crates.io/crates/semver/0.2.3). Rust
  crate versioning follows its interpretation of the
  [semver](http://semver.org) versioning scheme. Official [rust-lang]
  crate.

* [`tempdir-0.3.4`](https://crates.io/crates/tempdir/0.3.4). Another
  standard library reject. If you need to create temporary directories
  this is the official [rust-lang] way to do it.

* [`time-0.1.34`](https://crates.io/crates/time/0.1.34). This has got
  to be the worst of the [rust-lang] crates. It originates in the dark
  ages of Rust and limped along forever. If you need to get the time
  this can do it, but you won't like it.

* [`toml-0.1.34`](https://crates.io/crates/toml/0.1.34). [TOML](https://github.com/toml-lang/toml)
  is the format to use for configuration files (at least once you are
  ready to advace beyond simple [json serialization][json]). It is the
  format for configuring Cargo (via
  [Cargo.toml](http://doc.crates.io/manifest.html)), and the Rust
  implementation is maintained by master wizard [Alex
  Crichton](https://github.com/alexcrichton).

* [`url-0.5.5`](https://crates.io/crates/url/0.5.5). The Rust URL
  parser and type created for [Servo](https://github.com/servo/servo), by
  [SimonSapin](https://github.com/simonsapin).

[rust-lang]: http://github.com/rust-lang
[json]: http://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html#using-autoserialization

# Contributing

**stdx** favors crates that have been 'battle-tested', this includes old
and unloved crates like [`libc`], crates that are used by the official
toolchain (which are maintained and highly-compatible if not always
beautiful), and crates that are otherwise popular and well maintained.

[`libc`]: https://github.com/rust-lang/libc

All crates must work on Rust's tier-1 platforms, currently x86 Linux,
OS X, and Windows.

All crates must be published to [crates.io](https://crates.io) along with documentation
link and declared license.

# License

**stdx** and the crates it links to are licensed under various
[permissive, BSD-like][perm] licenses. In lay-terms these licenses
allow their code to be used and distributed freely, and are compatible
with [Rust's own license (MIT/Apache 2)][rustlice].

**stdx** itself is dual MIT/Apache 2 licensed, like Rust, and the
copyright is owned by its contributors.

[perm]: https://en.wikipedia.org/wiki/Permissive_free_software_licence
[rustlice]: https://github.com/rust-lang/rust/blob/master/COPYRIGHT

