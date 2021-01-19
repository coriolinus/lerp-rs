//! Macros for the [lerp-rs] crate
//! [lerp-rs]: https://github.com/coriolinus/lerp-rs

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

mod derive;

/// Automatically derive the Lerp trait for any struct fields that derive lerp
///
/// This derive implementation will lerp each field of the struct independently
/// and assumes a generic implementation of Lerp over `Float` types. If any
/// of the fields is generic only over one of the float values (f32, f64) that
/// can be specified by the `#[lerp(f32)]` or `#[lerp(f64)]` attributes respectively.
///
/// If you would like for the lerp implementation to ignore a field (or if it does
/// not derive lerp) you can use the `#[lerp(skip)]` or `#[lerp(ignore)]` attributes
/// which will produce the value, untouched from the left value.
///
/// Not all types are supported in this derive macro. See [the github issue] for
/// discussion and more information
///
/// [the github issue]: https://github.com/coriolinus/lerp-rs/issues/6
#[proc_macro_derive(Lerp, attributes(lerp))]
pub fn lerp_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    derive::lerp_derive_internal(&input)
        .unwrap_or_else(|err| {
            let err = err.to_compile_error();
            let name = input.ident;

            // On a compile error, produce the most generic implementation to remove
            // any type errors about the trait not being implemented, and instead move
            // the focus to the error produced by the macro
            quote! {
                #err

                #[automatically_derived]
                impl<F: ::lerp::num_traits::Float> Lerp<F> for #name {
                    fn lerp(self, other: Self, t: F) -> Self {
                        unreachable!()
                    }
                }
            }
        })
        .into()
}
