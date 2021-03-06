Multiversion
============
[![Build Status](https://img.shields.io/github/workflow/status/calebzulawski/multiversion/CI)](https://github.com/calebzulawski/multiversion/actions)
[![Rustc Version 1.31+](https://img.shields.io/badge/rustc-1.31+-lightgray.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)
[![License](https://img.shields.io/crates/l/multiversion)](https://crates.io/crates/multiversion)
[![Crates.io](https://img.shields.io/crates/v/multiversion)](https://crates.io/crates/multiversion)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/multiversion/0.3.0/multiversion/)

Function multiversioning attribute macros for Rust.

## What is function multiversioning?
Many CPU architectures have a variety of instruction set extensions that provide additional functionality.
Common examples are single instruction, multiple data (SIMD) extensions such as SSE and AVX on x86/x86-64 and NEON on ARM/AArch64.
When available, these extended features can provide significant speed improvements to some functions.
These optional features cannot be haphazardly compiled into programs--executing an unsupported instruction will result in a crash.

**Function multiversioning** is the practice of compiling multiple versions of a function with various features enabled and safely detecting which version to use at runtime.

## Features
* Dynamic dispatching, using runtime CPU feature detection
* Static dispatching, avoiding repeated feature detection for nested multiversioned functions (and allowing inlining!)
* Support for generic and `async` functions

## Limitations
* No support for member functions or associated functions (or any other functions not at module-level)
* No support for `impl Trait`

## Example
Automatic function multiversioning with the `target_clones` attribute, similar to GCC's `target_clones` attribute:
```rust
use multiversion::target_clones;

#[target_clones("[x86|x86_64]+avx", "x86+sse")]
fn square(x: &mut [f32]) {
    for v in x {
        *v *= *v;
    }
}
```

Manual function multiversioning with the `multiversion` and `target` attributes:
```rust
use multiversion::{multiversion, target};

#[target("[x86|x86_64]+avx")]
unsafe fn square_avx(x: &mut [f32]) {
    for v in x {
        *v *= *v;
    }
}

#[target("x86+sse")]
unsafe fn square_sse(x: &mut [f32]) {
    for v in x {
        *v *= *v;
    }
}

#[multiversion(
    "[x86|x86_64]+avx" => unsafe square_avx,
    "x86+sse" => unsafe square_sse
)]
fn square(x: &mut [f32]) {
    for v in x {
        *v *= *v;
    }
}
```

## License
Multiversion is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
