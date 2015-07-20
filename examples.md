# stdxamples

Usage examples of the libraries in **stdx**.

Each example has an implicit `stdx` import and `main` function. This
document is tested with [Rust
Skeptic](https://github.com/brson/rust-skeptic) using the following
[template](https://github.com/brson/rust-skeptic#skeptic-templates).

```rust,skeptic-template
extern crate stdx;

fn main() {{
   {}
}}
```

## Create a random integer

```rust
use stdx::rand::random;

let _r: i32 = random();
```
