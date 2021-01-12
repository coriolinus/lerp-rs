//! Macros for the [lerp-rs] crate
//! [lerp-rs]: https://github.com/coriolinus/lerp-rs

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

mod derive;

/// Automatically derive the Lerp trait for any struct with homogeneous float fields,
/// either f64 or f32. They can not mix
///
/// This derive implementation will lerp each field of the struct independently
#[proc_macro_derive(Lerp, attributes(lerp))]
pub fn lerp_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    derive::lerp_derive_internal(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
