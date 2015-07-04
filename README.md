# The missing batteries of Rust

Rust is a blank canvas. You can create anything with Rust that your
big beautiful brain can imagine. When you master Rust you will Be a
Better Person.

But how do you create a fucking random number?

**stdx** is a curated collection of [well-regarded Rust
crates][stdx-current] for typical programming tasks, as well as a
single crate that combines them together in a customizable way for
convenience. *If you are a Rust newbie,* **stdx** *reveals the most
legendary Crates in Rustendom that [everybody
else](http://rustaceans.org/) already knows about!* Read these words
and step up to the next level in the [Temple of
Rust](http://brson.github.io/temple-of-rust).

***Warning: stdx does not work yet. It is still mostly conceptual.***

[![Build Status](https://travis-ci.org/brson/stdx.svg?branch=master)](https://travis-ci.org/rust-lang/brson/stdx)

# Getting started

***Warning: Don't do this. It does not work. Instead read the [recommended crates][stdx-current] and link to them [like the pros do](http://doc.crates.io/crates-io.html#using-crates.io-based-crates).***

Add this to your Cargo.toml:

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

fn main() {
    use stdx::rand::{self, rand::Rng};

    let mut rng = rand::thread_rng();
    println!("{}", rng.gen::<i32>())
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

* [`bitflags-0.3.0`](https://crates.io/crates/bitflags/0.3.0) - The
  only thing this crate does is export the
  [`bitflags!`](http://doc.rust-lang.org/bitflags/bitflags/macro.bitflags!.html#example)
  macro, but it's a heckuva-useful macro. `bitflags!` produces
  typesafe bitmasks, types with named values that are efficiently
  packed together as bits to express sets of options.

* [`docopt-0.6.67`](https://crates.io/crates/docopt/0.6.67) - The
  preferred way to parse command line options in Rustland. Originally
  created by Rust API design pioneer, [BurntSushi], it is used by
  Cargo and many other projects, but notably not by rustc and rustdoc,
  which use the older [`getopts`](https://crates.io/crates/getopts)
  crate.

* [`env_logger-0.3.1`](https://crates.io/crates/env_logger/0.3.1) - In
  conjunction with the [`log`](https://crates.io/crates/log) crate,
  enables the output of logs to the console via the
  [`RUST_LOG`](http://doc.rust-lang.org/log/env_logger/index.html#enabling-logging)
  environment variable. This was a feature of the Rust language and
  runtime since ancient times, but is now implemented in an external
  library, and is still the most popular way to log information about
  what your Rust program is doing.

* [`flate2-0.2.7`](https://crates.io/crates/flate2/0.2.7) - Basic
  [deflate](https://en.wikipedia.org/wiki/DEFLATE) compression and
  decompression, via bindings to the [miniz
  library](https://code.google.com/p/miniz/).

* [`hyper-0.6.1`](https://crates.io/crates/hyper/0.6.1) - The most
  full-featured pure-Rust implementation of HTTP. Trusted by
  [Servo](https://github.com/servo/servo) and maintained by Mozilla's
  [seanmonstar](https://github.com/seanmonstar) so it's pretty solid.

* [`lazy_static-0.1.11`](https://crates.io/crates/lazy_static/0.1.11)

* [`libc-0.1.8`](https://crates.io/crates/libc/0.1.8)

* [`log-0.3.1`](https://crates.io/crates/log/0.3.1)

* [`num-0.1.25`](https://crates.io/crates/num/0.1.25)

* [`rand-0.3.8`](https://crates.io/crates/rand/0.3.8)

* [`regex-0.1.39`](https://crates.io/crates/regex/0.1.39)

* [`rustc-serialize-0.3.15`](https://crates.io/crates/rustc-serialize/0.3.15)

* [`semver-0.1.19`](https://crates.io/crates/semver/0.1.19)

* [`tempdir-0.3.4`](https://crates.io/crates/tempdir/0.3.4)

* [`time-0.1.3`](https://crates.io/crates/time/0.1.3)

* [`toml-0.1.21`](https://crates.io/crates/toml/0.1.21)

* [`url-0.2.35`](https://crates.io/crates/url/0.2.35)

[BurntSushi]: http://github.com/burntsushi

# Advanced details for the crate connoisseur

TODO: using crate features to enable crates selectively.

# Past batteries

Rust 1.2 will be the first release for which stdx exists. There are no
previous revisions yet.

# Contributing and policy

**stdx** favors crates that have been 'battle-tested', this includes old
and unloved crates like [`libc`], crates that are used by the official
toolchain (which are maintained and highly-compatible if not always
beautiful), and crates that are otherwise popular and well maintained.

[`libc`]: https://github.com/rust-lang/libc

All crates must work on Rust's tier-1 platforms, currently x86 Linux,
OS X, and Windows.

All crates must be published to crates.io along with documentation
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

