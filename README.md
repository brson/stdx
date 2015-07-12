# The missing batteries of Rust

[Rust](http://www.rust-lang.org) is a blank canvas. You can create
anything with Rust that your big beautiful brain can imagine. When you
master Rust you will Be a Better Person.

But how do you create a fucking random number?

Nobody even knows! (Well, that's not true, [the
gurus](https://github.com/ctjhoa/rust-learning#people) know, but
they're too often hiding themselves away in their guru-caves). [The
Rust Standard Library](http://doc.rust-lang.org/std/) is a mighty and
precious work of craftsmanship that will continue to serve our
grandparents when they are forging crates on the moon. It is not
though the comprehensive toolset they'll need to create serious moon
softwares, which often require such basic building blocks as, oh
... random numbers.

**stdx** then is a curated collection of [well-regarded Rust
crates][stdx-current] for typical programming tasks, as well as a
single crate that combines them together in a customizable way for
convenience. *If you are a Rust newbie,* **stdx** *reveals the most
legendary [Crates in Rustendom](https://crates.io) that [everybody
else](http://rustaceans.org/) already knows about! Read these words
and step up to the next level in the [Temple of
Rust](http://brson.github.io/temple-of-rust).*

***Warning: stdx does not work yet. It is still mostly conceptual.***

[![Build Status](https://travis-ci.org/brson/stdx.svg?branch=master)](https://travis-ci.org/rust-lang/brson/stdx)

# Getting started

***Warning: Don't do this. It does not work. Instead read the [recommended crates][stdx-current] and link to them [like the pros do](http://doc.crates.io/crates-io.html#using-crates.io-based-crates).***

Add this to your [Cargo.toml file](http://doc.crates.io/manifest.html):

```toml
[dependencies]
stdx = "0.102"
```

Then add this to the top of your crate source:

```rust
extern crate stdx;
```

Now you can access all of the crates in the revision of **stdx** that
[corresponds to the 1.2 release][stdx-102] of the Rust compiler and
language, and which will also work with all future 1.x revisions<sup>†</sup> of
the Rust language!

Check it out:

```rust
extern crate stdx;

use stdx::rand::random;

fn main() {
    println!("{}", random::<i32>())
}
```

That's how you create a fucking random number.

† *Well, we'll try anyway. These libraries are
[popular](https://crates.io/crates?sort=downloads) and [the Rust
team](http://www.rust-lang.org/team.html) will do their best not to
break them.*

# Current batteries
[stdx-current]: #current-batteries
[stdx-102]: #current-batteries

The current revision of **stdx** is [`stdx-0.102.0`](https://crates.io/crates/stdx/0.102.0), which corresponds to
the Rust stable 1.2 release.

* [`bitflags-0.3.0`](https://crates.io/crates/bitflags/0.3.0). The
  only thing this crate does is export the
  [`bitflags!`](http://doc.rust-lang.org/bitflags/bitflags/macro.bitflags!.html#example)
  macro, but it's a heckuva-useful macro. `bitflags!` produces
  typesafe bitmasks, types with named values that are efficiently
  packed together as bits to express sets of options. Official
  [rust-lang] crate.

* [`docopt-0.6.67`](https://crates.io/crates/docopt/0.6.67). The
  preferred way to parse command line options in Rustland. Originally
  created by Rust API design pioneer,
  [BurntSushi](http://github.com/burntsushi), it is used by Cargo and
  many other projects, but notably not by rustc and rustdoc, which use
  the older [`getopts`](https://crates.io/crates/getopts) crate.

* [`env_logger-0.3.1`](https://crates.io/crates/env_logger/0.3.1). In
  conjunction with the [`log`](https://crates.io/crates/log) crate,
  enables the output of logs to the console via the
  [`RUST_LOG`](http://doc.rust-lang.org/log/env_logger/index.html#enabling-logging)
  environment variable. This was a feature of the Rust language and
  runtime since ancient times, but is now implemented in an external
  library, and is still the most popular way to log information about
  what your Rust program is. Official [rust-lang] crate.

* [`flate2-0.2.7`](https://crates.io/crates/flate2/0.2.7)
  *(Optional)*. Basic [deflate](https://en.wikipedia.org/wiki/DEFLATE) compression and decompression, via bindings to the [miniz
  library](https://code.google.com/p/miniz/).
  **To enable**: use the **flate2** feature.

* [`hyper-0.6.1`](https://crates.io/crates/hyper/0.6.1) *(Optional)*.
  The most full-featured pure-Rust implementation of HTTP. Trusted by
  [Servo](https://github.com/servo/servo) and maintained by Mozilla's
  [seanmonstar](https://github.com/seanmonstar) so it's pretty solid.
  **To enable**: use the **hyper** feature.

* [`itertools-0.3.21`](https://crates.io/crates/itertools/0.3.21).
  When it comes to iterators, this crate has everything *including*
  the kitchen sink (in the form of the `batching` adaptor).
  Highlights include `dedup`, `group_by`, `mend_slices`, `merge`,
  `join` and more.

* [`lazy_static-0.1.11`](https://crates.io/crates/lazy_static/0.1.11).
  Global state is one of those things Rust doesn't do so well. In
  particular there is no ['life before
  main'](https://isocpp.org/wiki/faq/ctors#static-init-order) in Rust,
  so it's not possible to write a programmatic constructor for a
  global value that will be run at startup. Instead, Rust prefers lazy
  execution for global initialization, and the
  [`lazy_static!`](http://rust-ci.org/Kimundi/lazy-static.rs/doc/lazy_static/)
  macro does just that.

* [`libc-0.1.8`](https://crates.io/crates/libc/0.1.8). If you need to
  talk to foreign code, you need this crate. It contains declarations
  for a grab bag of C types and functions that are correct for the
  variety of compilers and platforms that Rust runs on. This crate is
  a *notorious mess* design-wise, but it has endured for years as the
  foundation Rust uses to talk to the outside world. Official
  [rust-lang] crate.

* [`log-0.3.1`](https://crates.io/crates/log/0.3.1). The most common
  way to perform basic logging in Rust, with the `error!`, `warn!`,
  `info!`, and `debug!` macros, always used in conjunction with the
  [`env_logger`](https://crates.io/crates/env_logger) crate.
  Official [rust-lang] crate.

* [`num-0.1.25`](https://crates.io/crates/num/0.1.25). Big integers,
  rational numbers, complex numbers, and a 'numeric tower' of numeric
  traits. This is another rust-lang crate that has persisted through
  Rust's evolution but is not designed well enough for the standard
  library. It is though presently the most common way to access the
  functionality it provides. Official [rust-lang] crate.

* [`rand-0.3.8`](https://crates.io/crates/rand/0.3.8). Random number
  generators. The defaults are cryptographically strong. Official
  [rust-lang] crate.

* [`regex-0.1.39`](https://crates.io/crates/regex/0.1.39). Another
  [BurntSushi](http://github.com/burntsushi) joint, this a very
  performant regular expression implementation that [stomps the
  competition](http://benchmarksgame.alioth.debian.org/u64/performance.php?test=regexdna)
  in some benchmarks. Though influenced by the highly-regarded
  [RE2](https://github.com/google/re2) engine, it omits backreferences
  and arbitrary lookahead in order to have predictable worst-case
  performance. Official [rust-lang] crate.

* [`rustc-serialize-0.3.15`](https://crates.io/crates/rustc-serialize/0.3.15).
  Another crate with a storied history, it was designed long ago to be
  Rust's solution for serialization, but time and better judgement
  consigned it to the Rust junkyard. Curiously, this is the only
  external crate that `rustc` has explicit knowledge of, since
  `#[deriving(RustcEncodable, RustcDecodable)]` is hard-coded into the
  compiler, and at the time the `rustc-serialize` crate was demoted it
  was deemed too inconvenient to users to remove the deriving
  mode. [Serde](https://github.com/erickt/rust-serde) is a more
  modern, and faster alternative, though it is yet inconvenient to use
  ... because it doesn't support `#[deriving]`! Official [rust-lang]
  crate.

* [`semver-0.1.19`](https://crates.io/crates/semver/0.1.19)

* [`tempdir-0.3.4`](https://crates.io/crates/tempdir/0.3.4)

* [`time-0.1.30`](https://crates.io/crates/time/0.1.30)

* [`toml-0.1.21`](https://crates.io/crates/toml/0.1.21)

* [`url-0.2.35`](https://crates.io/crates/url/0.2.35)

[rust-lang]: http://github.com/rust-lang

# Advanced details for the crate connoisseur

Some of the crates in **stdx** are optional, and must be enabled
explicitly in your manifest.  They might be disabled because they
do not work well on all platforms, because they are very large, or
because they require special environment configuration to build.

You can enable these crates by listing the ones you want like so:

```toml
[dependencies.stdx]
version = "0.102"
features = ["flate2", "hyper"]
```

**Note**: Having a `[dependencies.stdx]` section *replaces* the short
`stdx = "..."` line.

You can also use `"all"` to just enable everything.

If you want to go the other way, you can also select *just* the set of crates you want.  For example, if you only want the `env_logger`, `log`, and `num` crates, you could use the following:

```toml
[dependencies.stdx]
version = "0.102"
default-features = false
features = ["env_logger", "log", "num"]
```

There are more details about how features work in the
[Cargo documentation](http://doc.crates.io/manifest.html#the-[features]-section).

# Past batteries

Rust 1.2 will be the first release for which **stdx** exists. There are no
previous revisions yet.

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

