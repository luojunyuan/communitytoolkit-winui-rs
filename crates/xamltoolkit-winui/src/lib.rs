//! Rust projection experiment for the native XamlToolkit.WinUI WinRT component.
//!
//! This crate does not implement toolkit functionality. It generates Rust bindings from
//! `XamlToolkit.WinUI.winmd` at build time using `windows-bindgen`.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use XamlToolkit::WinUI::*;
