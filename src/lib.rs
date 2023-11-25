#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// wasm bindings for tauri's provided js functions
#[cfg(target_family = "wasm")]
pub mod bindings;
/// wrapped bindings for easier use in the generated wasm commands
#[cfg(target_family = "wasm")]
pub mod command;
/// related generic struct and functions for autogenerated listen functions
#[cfg(all(target_family = "wasm", feature = "listen"))]
pub mod listen;

pub use tauri_interop_macro::*;
