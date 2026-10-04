//! Rust projection for the native XamlToolkit.WinUI.Converters WinRT component.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use XamlToolkit::WinUI::Converters;
pub use XamlToolkit::WinUI::Converters::*;
