use proc_macro2::TokenStream;
use quote::{quote};
use syn::{Error, Field, Fields, Index, ItemStruct, Result, spanned::Spanned};

pub fn lerp_derive_internal(input: ItemStruct) -> Result<TokenStream> {
    let name = &input.ident;

    match input.fields {
        Fields::Named(fields) => {
            if let Some(Field { ty, .. }) = fields.named.first() {
                let fields = fields.named.iter().map(|f| {
                    if let Some(name) = f.ident.as_ref() {
                        Ok(quote! {
                            #name: self.#name.lerp(other.#name, t)
                        })
                    } else {
                        Err(Error::new(f.ident.span(), "All fields must be named in the struct"))
                    }
                }).collect::<Result<Vec<_>>>()?;

                Ok(quote! {
                    #[automatically_derived]
                    impl Lerp<#ty> for #name {
                        fn lerp(self, other: Self, t: #ty) -> Self {
                            Self {
                                #(#fields),*
                            }
                        }
                    }
                })
            } else {
                Err(Error::new(fields.named.span(), "Struct must have one or more fields"))
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

                Ok(quote! {
                    #[automatically_derived]
                    impl Lerp<#ty> for #name {
                        fn lerp(self, other: Self, t: #ty) -> Self {
                            Self (
                                #(#fields),*
                            )
                        }
                    }
                })
            } else {
                Err(Error::new(fields.unnamed.span(), "Struct must have one or more fields"))
            }
        }
        _ => Err(Error::new(input.span(), "Struct must have fields")),
    }
}
