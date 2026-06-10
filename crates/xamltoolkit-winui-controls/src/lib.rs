//! Rust projection for the native XamlToolkit.WinUI.Controls WinRT component.
//!
//! This crate generates bindings from `XamlToolkit.WinUI.Controls.winmd` at build time
//! using `windows-bindgen`.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
