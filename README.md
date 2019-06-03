# `slang-rs`

Provides the following crates:

## `slang-sys` ![crates.io](https://img.shields.io/crates/v/slang-sys.svg)  ![docs.rs](https://docs.rs/slang-sys/badge.svg)

FFI bindings to a release of [Slang](https://github.com/shader-slang/slang).

This crate is usable on Windows (x86/x64) and Linux (x64).

## `slang` ![crates.io](https://img.shields.io/crates/v/slang.svg) ![docs.rs](https://docs.rs/slang/badge.svg)

Safe Rust wrapper to `slang-sys`.

This crate is usable, but missing some functionality (most notably the reflection API).

## `shaders`

Extends `slang` with [`spirv_cross`](https://crates.io/crates/spirv_cross) usage to target DirectX11, non-Vulkan OpenGL, Metal, etc.

This crate is experimental.