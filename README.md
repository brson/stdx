# The missing batteries of Rust

**stdx** is a project to collect the best Rust crates and validate
that they work together on the platforms that Rust supports.

## Current crates

The current revision of **stdx** is "0.115.0", which corresponds to
the Rust stable 1.15 release.


### `bitflags = "0.7.0"`

&emsp; [📖 docs][d-bitflags] &emsp; [📦 crates.io][c-bitflags]

[d-bitflags]: https://docs.rs/bitflags/0.7.0/bitflags/
[c-bitflags]: https://crates.io/crates/bitflags/0.7.0

The only thing this crate does is export the
[`bitflags!`](http://doc.rust-lang.org/bitflags/bitflags/macro.bitflags!.html#example)
macro, but it's a heckuva-useful macro. `bitflags!` produces typesafe
bitmasks, types with named values that are efficiently packed together
as bits to express sets of options. Official [rust-lang] crate.


### `clap = "2.20.0"`

&emsp; [📖 docs][d-clap] &emsp; [📦 crates.io][c-clap]

[d-clap]: https://docs.rs/clap/2.20.0/clap/
[c-clap]: https://crates.io/crates/clap/2.20.0

Clap is a command line argument parser that is easy to
use and is highly configurable.

**Alternatives**: [docopt]

[docopt]: https://docs.rs/docopt


### `env_logger = "0.4.0"`

&emsp; [📖 docs][d-env_logger] &emsp; [📦 crates.io][c-env_logger]

[d-env_logger]: https://docs.rs/env_logger/0.4.0/env_logger/
[c-env_logger]: https://crates.io/crates/env_logger/0.4.0

In conjunction with the [`log`](https://crates.io/crates/log) crate,
enables the output of logs to the console via the
[`RUST_LOG`](http://doc.rust-lang.org/log/env_logger/index.html#enabling-logging)
environment variable. This was a feature of the Rust language and
runtime since ancient times, but is now implemented in an external
library, and is still the most popular way to log information about
what your Rust program is. Official [rust-lang] crate.


### `itertools = "0.5.9"`

&emsp; [📖 docs][d-itertools] &emsp; [📦 crates.io][c-itertools]

[d-itertools]: https://docs.rs/itertools/0.5.9/itertools/
[c-itertools]: https://crates.io/crates/itertools/0.5.9

When it comes to iterators, this crate has everything *including* the
kitchen sink (in the form of the `batching` adaptor).  Highlights
include `dedup`, `group_by`, `mend_slices`, `merge`, `join` and more.


### `lazy_static = "0.2.2"`

&emsp; [📖 docs][d-lazy_static] &emsp; [📦 crates.io][c-lazy_static]

[d-lazy_static]: https://docs.rs/env_logger/0.2.2/env_logger/
[c-lazy_static]: https://crates.io/crates/env_logger/0.2.2

Global state is one of those things Rust doesn't do so well. In
particular there is no ['life before
main'](https://isocpp.org/wiki/faq/ctors#static-init-order) in Rust,
so it's not possible to write a programmatic constructor for a global
value that will be run at startup. Instead, Rust prefers lazy
execution for global initialization, and the
[`lazy_static!`](http://rust-ci.org/Kimundi/lazy-static.rs/doc/lazy_static/)
macro does just that.


### `libc = "0.2.18"`

&emsp; [📖 docs][d-libc] &emsp; [📦 crates.io][c-libc]

[d-libc]: https://docs.rs/libc/0.2.18/libc/
[c-libc]: https://crates.io/crates/libc/0.2.18

If you need to talk to foreign code, you need this crate. It contains
declarations for a grab bag of C types and functions that are correct
for the variety of compilers and platforms that Rust runs on. This
crate is a *notorious mess* design-wise, but it has endured for years
as the foundation Rust uses to talk to the outside world. Official
[rust-lang] crate.


### `log = "0.3.6"`

&emsp; [📖 docs][d-log] &emsp; [📦 crates.io][c-log]

[d-log]: https://docs.rs/log/0.3.6/log/
[c-log]: https://crates.io/crates/log/0.3.6

The most common way to perform basic logging in Rust, with the
`error!`, `warn!`, `info!`, and `debug!` macros, always used in
conjunction with the
[`env_logger`](https://crates.io/crates/env_logger) crate.  Official
[rust-lang] crate.


### `num = "0.1.36"`

&emsp; [📖 docs][d-num] &emsp; [📦 crates.io][c-num]

[d-num]: https://docs.rs/num/0.1.36/num/
[c-num]: https://crates.io/crates/num/0.1.36

Big integers, rational numbers, complex numbers, and a 'numeric tower'
of numeric traits. This is another rust-lang crate that has persisted
through Rust's evolution but is not designed well enough for the
standard library. It is though presently the most common way to access
the functionality it provides.


### `rand = "0.3.15"`

&emsp; [📖 docs][d-rand] &emsp; [📦 crates.io][c-rand]

[d-rand]: https://docs.rs/rand/0.3.15/rand/
[c-rand]: https://crates.io/crates/rand/0.3.15

Random number generators. The defaults are cryptographically
strong. Official [rust-lang] crate.


### `regex = "0.2.1"`

&emsp; [📖 docs][d-regex] &emsp; [📦 crates.io][c-regex]

[d-regex]: https://docs.rs/regex/0.2.1/regex/
[c-regex]: https://crates.io/crates/regex/0.2.1

Another [BurntSushi](http://github.com/burntsushi) joint, this a very
performant regular expression implementation that [stomps the
competition](http://benchmarksgame.alioth.debian.org/u64/performance.php?test=regexdna)
in some benchmarks. Influenced by the highly-regarded
[RE2](https://github.com/google/re2) engine, it omits backreferences
and arbitrary lookahead in order to have predictable worst-case
performance. Official [rust-lang] crate.


### `rustc-serialize = "0.3.22"`

&emsp; [📖 docs][d-rustc-serialize] &emsp; [📦 crates.io][c-rustc-serialize]

[d-rustc-serialize]: https://docs.rs/rustc-serialize/0.3.22/rustc-serialize/
[c-rustc-serialize]: https://crates.io/crates/rustc-serialize/0.3.22

Another crate with a storied history, it was designed long ago to be
Rust's solution for serialization, but time and better judgement
consigned it to the Rust junkyard. Curiously, this is the only
external crate that `rustc` has explicit knowledge of, since
`#[derive(RustcEncodable, RustcDecodable)]` is hard-coded into the
compiler, and at the time the `rustc-serialize` crate was demoted it
was deemed too inconvenient to users to remove the deriving
mode. [Serde](https://github.com/serde-rs/serde) is a more modern, and
faster alternative. Official [rust-lang] crate.


### `semver = "0.5.1"`

&emsp; [📖 docs][d-semver] &emsp; [📦 crates.io][c-semver]

[d-semver]: https://docs.rs/semver/0.5.1/semver/
[c-semver]: https://crates.io/crates/semver/0.5.1

Rust crate versioning follows its interpretation of the
[semver](http://semver.org) versioning scheme. Official [rust-lang]
crate.


### `tempdir = "0.3.5"`

&emsp; [📖 docs][d-tempdir] &emsp; [📦 crates.io][c-tempdir]

[d-tempdir]: https://docs.rs/tempdir/0.3.5/tempdir/
[c-tempdir]: https://crates.io/crates/tempdir/0.3.5

Another standard library reject. If you need to create temporary
directories this is the official [rust-lang] way to do it.


### `time = "0.1.36"`

&emsp; [📖 docs][d-time] &emsp; [📦 crates.io][c-time]

[d-time]: https://docs.rs/time/0.1.36/time/
[c-time]: https://crates.io/crates/time/0.1.36

This has got to be the worst of the [rust-lang] crates. It originates
in the dark ages of Rust and limped along forever. If you need to get
the time this can do it, but you won't like it.


### `toml = "0.2.1"`

&emsp; [📖 docs][d-toml] &emsp; [📦 crates.io][c-toml]

[d-toml]: https://docs.rs/toml/0.2.1/toml/
[c-toml]: https://crates.io/crates/toml/0.2.1

[TOML](https://github.com/toml-lang/toml) is the format to use for
configuration files (at least once you are ready to advace beyond
simple [json serialization][json]). It is the format for configuring
Cargo (via [Cargo.toml](http://doc.crates.io/manifest.html)), and the
Rust implementation is maintained by master wizard [Alex
Crichton](https://github.com/alexcrichton).


### `url = "1.3.0"`

&emsp; [📖 docs][d-url] &emsp; [📦 crates.io][c-url]

[d-url]: https://docs.rs/url/1.3.0/url/
[c-url]: https://crates.io/crates/url/1.3.0

The Rust URL parser and type created for
[Servo](https://github.com/servo/servo), by
[SimonSapin](https://github.com/simonsapin).

[rust-lang]: http://github.com/rust-lang
[json]: http://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html#using-autoserialization


## Contributing

**stdx** favors crates that have been 'battle-tested', this includes old
and unloved crates like [`libc`], crates that are used by the official
toolchain (which are maintained and highly-compatible if not always
beautiful), and crates that are otherwise popular and well maintained.

[`libc`]: https://github.com/rust-lang/libc

All crates must work on Rust's tier-1 platforms, currently x86 Linux,
OS X, and Windows.

All crates must be published to [crates.io](https://crates.io) along with documentation
link and declared license.

## License

**stdx** and the crates it links to are licensed under various
[permissive, BSD-like][perm] licenses. In lay-terms these licenses
allow their code to be used and distributed freely, and are compatible
with [Rust's own license (MIT/Apache 2)][rustlice].

**stdx** itself is dual MIT/Apache 2 licensed, like Rust, and the
copyright is owned by its contributors.

[perm]: https://en.wikipedia.org/wiki/Permissive_free_software_licence
[rustlice]: https://github.com/rust-lang/rust/blob/master/COPYRIGHT

