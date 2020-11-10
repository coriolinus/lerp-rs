use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Field, Fields, Index, ItemStruct};

#[proc_macro_derive(Lerp)]
pub fn lerp_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident;

    TokenStream::from(match input.fields {
        Fields::Named(fields) => {
            if let Some(Field { ty, .. }) = fields.named.first() {
                let fields = fields.named.iter().map(|f| {
                    if let Some(name) = f.ident.as_ref() {
                        quote! {
                            #name: self.#name.lerp(other.#name, t)
                        }
                    } else {
                        quote_spanned! {f.ident.span()=>
                            compile_error!("All fields must be named in the struct");
                        }
                    }
                });

                quote! {
                    #[automatically_derived]
                    impl Lerp<#ty> for #name {
                        fn lerp(self, other: Self, t: #ty) -> Self {
                            Self {
                                #(#fields),*
                            }
                        }
                    }
                }
            } else {
                quote_spanned! {fields.named.span()=>
                    compile_error!("Struct must have one or more fields");
                }
            }
        }
        Fields::Unnamed(fields) => {
            if let Some(Field { ty, .. }) = fields.unnamed.first() {
                let fields = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let name = Index::from(i);

                    quote! {
                        self.#name.lerp(other.#name, t)
                    }
                });

                quote! {
                    #[automatically_derived]
                    impl Lerp<#ty> for #name {
                        fn lerp(self, other: Self, t: #ty) -> Self {
                            Self (
                                #(#fields),*
                            )
                        }
                    }
                }
            } else {
                quote_spanned! {fields.unnamed.span()=>
                    compile_error!("Struct must have one or more fields");
                }
            }
        }
        _ => quote_spanned! {input.span()=>
            compile_error!("Struct must have fields");
        },
    })
}
