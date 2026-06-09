# About

This document describes various development styles and guidelines this project
follows.

## File Structure

Files should be structured as follows:
```rust
// imports
use crate::git;
// ...

// module declarations
mod app;
// ...

// public types
pub struct MyStruct { ... };

// private types
enum Tag { ... };

// public API (pub functions, possibly within `impl` blocks)
pub fn foo(bar: f32) { ... };

// private functions
```

Sections can optionally be separated by comment blocks:
```rust
// ====================================================================================================
// Top-level types
// ====================================================================================================
```
