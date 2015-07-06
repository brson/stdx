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

* [`itertools-0.3.21`](https://crates.io/crates/itertools/0.3.21).
  When it comes to iterators, this crate has everything *including*
  the kitchen sink (in the form of the `batching` adaptor).
  Highlights include `dedup`, `group_by`, `mend_slices`, `merge`,
  `join` and more.

* [`lazy_static-0.1.11`](https://crates.io/crates/lazy_static/0.1.11)

* [`libc-0.1.8`](https://crates.io/crates/libc/0.1.8)

* [`log-0.3.1`](https://crates.io/crates/log/0.3.1)

* [`num-0.1.25`](https://crates.io/crates/num/0.1.25)

* [`rand-0.3.8`](https://crates.io/crates/rand/0.3.8)

* [`regex-0.1.39`](https://crates.io/crates/regex/0.1.39)

* [`rustc-serialize-0.3.15`](https://crates.io/crates/rustc-serialize/0.3.15)

* [`semver-0.1.19`](https://crates.io/crates/semver/0.1.19)

* [`tempdir-0.3.4`](https://crates.io/crates/tempdir/0.3.4)

* [`time-0.1.30`](https://crates.io/crates/time/0.1.30)

* [`toml-0.1.21`](https://crates.io/crates/toml/0.1.21)

* [`url-0.2.35`](https://crates.io/crates/url/0.2.35)

[rust-lang]: http://github.com/rust-lang

# Advanced details for the crate connoisseur

Though they are all enabled by default, all of the crates included in
**stdx** are optional, allowing you to select *just* the set of crates
you want. For example, if you only want the `env_logger`, `log`, and
`num` crates, you could use the following:

```toml
[dependencies.stdx]
version = "0.102"
default-features = false
features = ["env_logger", "log", "num"]
```

**Note**: Having a `[dependencies.stdx]` section *replaces* the short
`stdx = "..."` line.

You can also use `"all"` to just enable everything.

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

