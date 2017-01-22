# `stdx` - The missing batteries of Rust

New to Rust and don't yet know what crates to use?
[**stdx has the best crates**](#about-stdx).

Current revision:  `stdx` 0.115.0, Rust 1.15, February 2, 2017.

| Feature                        | Crate                     |                    |
|--------------------------------|---------------------------|--------------------|
| Bitfields                      | [`bitflags = "0.7.0"`]    | [📖][d-bitflags]    |
| Byte order conversion          | [`byteorder = "1.0.0"`]   | [📖][d-byteorder]   |
| Date and time                  | [`chrono = "0.2.25"`]     | [📖][d-chrono]      |
| Command-line argument parsing  | [`clap = "2.20.0"`]       | [📖][d-clap]        |
| Error handling                 | [`error-chain = "0.8.1"`] | [📖][d-error-chain] |
| Iterator extensions            | [`itertools = "0.5.9"`]   | [📖][d-itertools]   |
| JSON                           | [`json = "0.11.5"`]       | [📖][d-json]        |
| Global initialization          | [`lazy_static = "0.2.2"`] | [📖][d-lazy_static] |
| C interop                      | [`libc = "0.2.18"`]       | [📖][d-libc]        |
| Logging                        | [`log = "0.3.6"`]         | [📖][d-log]         |
| Multidimensional arrays        | [`ndarray = "0.7.2"`]     | [📖][d-ndarray]     |
| Big, rational, complex numbers | [`num = "0.1.36"`]        | [📖][d-num]         |
| Random numbers                 | [`rand = "0.3.15"`]       | [📖][d-rand]        |
| Parallel iteration             | [`rayon = "0.6.0"`]       | [📖][d-rayon]       |
| Regular expressions            | [`regex = "0.2.1"`]       | [📖][d-regex]       |
| HTTP client                    | [`reqwest = "0.3.0"`]     | [📖][d-reqwest]     |
| Serialization                  | [`serde = "0.9.0-rc2"`]   | [📖][d-serde]       |
| Temporary directories          | [`tempdir = "0.3.5"`]     | [📖][d-tempdir]     |
| Configuration files            | [`toml = "0.2.1"`]        | [📖][d-toml]        |
| URLs                           | [`url = "1.3.0"`]         | [📖][d-url]         |

[`bitflags = "0.7.0"`]: #bitflags
[`byteorder = "1.0.0"`]: #byteorder
[`chrono = "0.2.25"`]: #chrono
[`clap = "2.20.0"`]: #clap
[`error-chain = "0.8.1"`]: #error-chain
[`itertools = "0.5.9"`]: #itertools
[`json = "0.11.5"`]: #json
[`lazy_static = "0.2.2"`]: #lazy_static
[`libc = "0.2.18"`]: #libc
[`log = "0.3.6"`]: #log
[`ndarray = "0.7.2"`]: #ndarray
[`num = "0.1.36"`]: #num
[`rand = "0.3.15"`]: #rand
[`rayon = "0.6.0"`]: #rayon
[`regex = "0.2.1"`]: #regex
[`reqwest = "0.3.0"`]: #reqwest
[`serde = "0.9.0-rc2"`]: #serde
[`tempdir = "0.3.5"`]: #tempdir
[`toml = "0.2.1"`]: #toml
[`url = "1.3.0"`]: #url

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="bitflags"></a>
### `bitflags = "0.7.0"` &emsp; [📖][d-bitflags]

[d-bitflags]: https://docs.rs/bitflags/0.7.0/bitflags/

The only thing this crate does is export the
[`bitflags!`](http://doc.rust-lang.org/bitflags/bitflags/macro.bitflags!.html#example)
macro, but it's a heckuva-useful macro. `bitflags!` produces typesafe
bitmasks, types with named values that are efficiently packed together
as bits to express sets of options.

**Example**: [`bitflags.rs`]

[`bitflags.rs`]: examples/bitflags.rs

```rust
#[macro_use]
extern crate bitflags;

bitflags! {
    flags Flags: u32 {
        const FLAG_A       = 0b00000001,
        const FLAG_B       = 0b00000010,
        const FLAG_C       = 0b00000100,
        const FLAG_ABC     = FLAG_A.bits
                           | FLAG_B.bits
                           | FLAG_C.bits,
    }
}

fn main() {
    let e1 = FLAG_A | FLAG_C;
    let e2 = FLAG_B | FLAG_C;
    assert_eq!((e1 | e2), FLAG_ABC);   // union
    assert_eq!((e1 & e2), FLAG_C);     // intersection
    assert_eq!((e1 - e2), FLAG_A);     // set difference
    assert_eq!(!e2, FLAG_A);           // set complement
}
```

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="byteorder"></a>
### `byteorder = "1.0.0"` &emsp; [📖][d-byteorder]

[d-byteorder]: https://docs.rs/byteorder/0.7.0/byteorder/

Functions for converting between numbers and bytes, in
in little-endian, or big-endian orders.

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="chrono"></a>
### `chrono = "0.2.25"` &emsp; [📖][d-chrono]

[d-chrono]: https://docs.rs/chrono/0.2.25/chrono/

Date and time types.

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="clap"></a>
### `clap = "2.20.0"` &emsp; [📖][d-clap]

[d-clap]: https://docs.rs/clap/2.20.0/clap/

Clap is a command line argument parser that is easy to
use and is highly configurable.

**Alternatives**: [`docopt`]

[`docopt`]: https://docs.rs/docopt

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="error-chain"></a>
### `error-chain = "0.8.1"` &emsp; [📖][d-error-chain]

[d-error-chain]: https://docs.rs/error-chain/0.8.1/error_chain/

Rust programs that handle errors consistently are reliable programs.
Even after one understands [error handling] in Rust, it can be
difficult to grasp and implement its best practices. `error-chain`
helps you define your own error type that works with the `?` operator
to make error handling in Rust simple and elegant.

[error handling]: https://rust-lang.github.io/book/ch09-00-error-handling.html

**Alternatives**: [`quick-error`]

[`quick-error`]: https://docs.rs/quick-error/1.1.0/quick_error/

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="itertools"></a>
### `itertools = "0.5.9"` &emsp; [📖][d-itertools]

[d-itertools]: https://docs.rs/itertools/0.5.9/itertools/

When it comes to iterators, this crate has everything *including* the
kitchen sink (in the form of the `batching` adaptor).  Highlights
include `dedup`, `group_by`, `mend_slices`, `merge`, `join` and more.

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="json"></a>
### `json = "0.11.5"` &emsp; [📖][d-json]

[d-json]: https://docs.rs/json/0.11.5/json/

Access to [JSON], the "JavaScript Object Notation" format,
widely used for transmission and storage of data on the Internet.
This crate can be used for reading, writing, and manipulation
of arbitrary JSON; for simple serialization to Rust data structures,
use [`serde`](#serde) and `serde_json`.

[JSON]: http://json.org/

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="lazy_static"></a>
### `lazy_static = "0.2.2"` &emsp; [📖][d-lazy_static]

[d-lazy_static]: https://docs.rs/env_logger/0.2.2/env_logger/

Global state is one of those things Rust doesn't do so well. In
particular there is no ['life before
main'](https://isocpp.org/wiki/faq/ctors#static-init-order) in Rust,
so it's not possible to write a programmatic constructor for a global
value that will be run at startup. Instead, Rust prefers lazy
execution for global initialization, and the `lazy_static!`
macro does just that.

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="libc"></a>
### `libc = "0.2.18"` &emsp; [📖][d-libc]

[d-libc]: https://docs.rs/libc/0.2.18/libc/

If you need to talk to foreign code, you need this crate. It exports C
type and function definitions appropriate to each target platform Rust
supports. It defines the standardized C features that are common
across all platforms as well as non-standard features specific to the
platform C libraries. For more platform-specific FFI definitions
see [`nix`] and [`winapi`].

**Supplemental crates**: [`nix`], [`winapi`]

[`nix`]: https://docs.rs/nix
[`winapi`]: https://docs.rs/winapi

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="log"></a>
### `log = "0.3.6"` &emsp; [📖][d-log]

[d-log]: https://docs.rs/log/0.3.6/log/

The most common way to perform basic logging in Rust, with the
`error!`, `warn!`, `info!`, and `debug!` macros. It is often
combined with the `env_logger` crate to get logging to the
console, controlled by the `RUST_LOG` environment variable.
This is the traditional logging crate used by `rustc`, and
its functionality was once built in to the language.

**Supplemental crates**: [`env_logger = "0.4.0"`]

[`env_logger = "0.4.0"`]: https://docs.rs/env_logger/0.4.0/env_logger/

**Alternatives**: [`slog`], [`log4rs`]

[`slog`]: https://docs.rs/slog
[`log4rs`]: https://docs.rs/log4rs

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="ndarray"></a>
### `ndarray = "0.7.2"` &emsp; [📖][d-ndarray]

[d-ndarray]: https://docs.rs/ndarray/0.7.2/ndarray/

The ndarray crate provides an N-dimensional container for general
elements and for numerics. The multidimensional array, otherwise known
as a "matrix", is a core data structure for numerical applications,
and Rust does not have one in the language or standard library.

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="num"></a>
### `num = "0.1.36"` &emsp; [📖][d-num]

[d-num]: https://docs.rs/num/0.1.36/num/

Big integers, rational numbers, complex numbers, and numeric
traits. This is a rust-lang crate that has persisted through Rust's
evolution but is somewhat unloved.

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="rand"></a>
### `rand = "0.3.15"` &emsp; [📖][d-rand]

[d-rand]: https://docs.rs/rand/0.3.15/rand/

Random number generators. The defaults are cryptographically strong.

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="rayon"></a>
### `rayon = "0.6.0"` &emsp; [📖][d-rayon]

[d-rayon]: https://docs.rs/rayon/0.6.0/rayon/

When people say that Rust makes parallelism easy, this
is why. Rayon provides parallel iterators that make
expressing efficient parallel operations simple
and foolproof.

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="regex"></a>
### `regex = "0.2.1"` &emsp; [📖][d-regex]

[d-regex]: https://docs.rs/regex/0.2.1/regex/

Rust's regular expressions are [fast], like Rust is fast. Part of
their power comes from a careful design that disallows back-references
and arbitrary lookahead, creating predictable worst-case performance.

[fast]: http://blog.burntsushi.net/ripgrep/

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="reqwest"></a>
### `reqwest = "0.3.0"` &emsp; [📖][d-reqwest]

[d-reqwest]: https://docs.rs/reqwest/0.3.0/reqwest/

A simple HTTP and HTTPS client.

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="serde"></a>
### `serde = "0.9.0-rc2"` &emsp; [📖][d-serde]

[d-serde]: https://docs.rs/serde/0.9.0-rc2/serde/

Serialization and deserialization of Rust datastructures is fast
and easy using the `serde` serialization framework. Simply
tag your data structures with `#[derive(Serialize, Deserialize)]`
and serde will automatically convert them between formats like
JSON, TOML, YAML, and more. To best understand serde, read
its documentation at [serde.rs].

**Supplemental crates**: [`serde_derive = "0.9.0-rc2"`],
                         [`serde_json = "0.9.0-rc1"`],
                         [`toml = "0.2.1"`]

**Alternatives**: [`rustc-serialize`]

[serde.rs]: https://serde.rs/
[`rustc-serialize`]: https://docs.rs/rustc-serialize
[`serde_derive = "0.9.0-rc2"`]: https://docs.rs/serde_derive/0.9.0-rc2/serde_derive
[`serde_json = "0.9.0-rc1"`]: https://docs.rs/serde_json/0.9.0-rc1/serde_json
[`toml = "0.2.1"`]: https://docs.rs/toml/0.2.1/toml

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="tempdir"></a>
### `tempdir = "0.3.5"` &emsp; [📖][d-tempdir]

[d-tempdir]: https://docs.rs/tempdir/0.3.5/tempdir/

The most common way to create temporary directories in Rust,
this crate was once part of the standard library.

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="toml"></a>
### `toml = "0.2.1"` &emsp; [📖][d-toml]

[d-toml]: https://docs.rs/toml/0.2.1/toml/

[TOML](https://github.com/toml-lang/toml) is a common format for
configuration files, like [Cargo.toml]. It's easy on the eyes, simple
to parse, and serializes from Rust types with [`serde`](#serde).

[Cargo.toml]: http://doc.crates.io/manifest.html

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


<a id="url"></a>
### `url = "1.3.0"` &emsp; [📖][d-url]

[d-url]: https://docs.rs/url/1.3.0/url/

The Rust URL parser and type, originally created
for [Servo].

[Servo]: https://servo.org

&nbsp;&NewLine;&nbsp;&NewLine;&nbsp;&NewLine;


## About `stdx`

Rust has a lovely and portable standard library, but it is not
featureful enough to write software of any great
sophistication. Compared to common platforms including Java, Python,
and Go, Rust's standard library is small.

In Rust, the libraries we use for even simple tasks live and evolve on
[crates.io]. This affords the Rust community freedom to experiment -
discovering the Rustiest solutions to even common problems can take
quite some iteration - but it also means that we're in for a slow
evolutionary process to converge around the best of those solutions. In
the meantime, you just have to know which crates to use for what.

`stdx` contains some of the most important crates in Rust. I mean
it. If Rust had a more expansive standard library, many of the `stdx`
crates would be in it. These are core elements of the crates ecosystem
that all Rusticians should be aware of.

[crates.io]: https://www.crates.io

## Selection criteria

The criteria for inclusion in `stdx` is conservative, and fuzzy. It's
mostly crates that I think are pretty super important, considering
criteria like

- universality of the feature
- portability
- quality
- interoperability with other stdx crates
- reliability of maintainers
- de-facto adoption
- historical context and precedent

`stdx` is focused on core features, crates that are quintessentially
Rust and make a part of many Rust programs. It is intentionally
limited for the sake of simplicity and ease of comprehension.

All crates must work on Rust's tier-1 platforms, currently x86 Linux,
OS X, and Windows.

## License

`stdx` and the crates it links to are licensed under various
[permissive, BSD-like][perm] licenses. In lay-terms these licenses
allow their code to be used and distributed freely, and are compatible
with [Rust's own license (MIT/Apache 2)][rustlice].

`stdx` itself is dual MIT/Apache 2 licensed, like Rust, and the
copyright is owned by its contributors.

[perm]: https://en.wikipedia.org/wiki/Permissive_free_software_licence
[rustlice]: https://github.com/rust-lang/rust/blob/master/COPYRIGHT

