use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Error, Field, Fields, Index, ItemStruct, Result, Token, Type, TypeGroup, TypeParen,
    TypePath, TypeReference,
};

enum LerpType {
    Type(TypePath),
    Generic,
    Skip,
}

#[derive(Default)]
struct LerpAttributes {
    skip: Option<()>,
    type_override: Option<TypePath>,
}

impl Parse for LerpAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let inputs = Punctuated::<TypePath, Token![,]>::parse_terminated(input)?;

        let mut skip = None;
        let mut type_override = None;

        for TypePath { path, .. } in inputs {
            if path.is_ident("skip") {
                if skip.is_none() {
                    return Err(Error::new(path.span(), "duplicate skip statement"));
                }

                skip = Some(());
            } else {
                if type_override.is_some() {
                    return Err(Error::new(path.span(), "duplicate occurrence of lerp type"));
                }

                type_override = Some(parse_quote! {
                    #path
                });
            }
        }

        Ok(Self {
            skip,
            type_override,
        })
    }
}

fn get_lerp_type(ty: &Type, attrs: &Vec<Attribute>) -> Result<LerpType> {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let attr = attrs
                .into_iter()
                .filter(|attr| attr.path.is_ident("lerp"))
                .collect::<Vec<_>>();

            let attr: LerpAttributes = match &attr[..] {
                [] => Ok(Default::default()),
                [Attribute { tokens, .. }] => {
                    syn::parse(proc_macro::TokenStream::from(tokens.clone()))
                }
                [_, overflow, ..] => Err(Error::new(
                    overflow.span(),
                    "found duplicate attribute on field, consolidate the attributes into one",
                )),
            }?;

            if attr.skip.is_some() {
                return Ok(LerpType::Skip);
            }

            Ok(if let Some(override_type) = attr.type_override {
                LerpType::Type(override_type)
            } else {
                if path.is_ident("f64") || path.is_ident("f32") {
                    LerpType::Type(parse_quote! { #path })
                } else {
                    LerpType::Generic
                }
            })
        }

        Type::Array(_) => {
            todo!()
        } // TODO: ?

        Type::Slice(_) => {
            todo!()
        } // TODO: ?

        Type::Tuple(_) => {
            todo!()
        } // TODO: ?

        // Recursively descend through groups and references
        Type::Group(TypeGroup { elem, .. })
        | Type::Paren(TypeParen { elem, .. })
        | Type::Reference(TypeReference { elem, .. }) => get_lerp_type(elem, attrs),

        _ => Err(Error::new(ty.span(), "Unsupported type")),
    }
}

pub fn lerp_derive_internal(input: ItemStruct) -> Result<TokenStream> {
    let name = &input.ident;

    match input.fields {
        Fields::Named(fields) => {
            let fields = fields
                .named
                .iter()
                .map(|f| {
                    if let Field {
                        attrs,
                        ident: Some(name),
                        ty,
                        ..
                    } = f
                    {
                        let lerp_type = get_lerp_type(ty, attrs)?;

                        Ok(match lerp_type {
                            LerpType::Type(ty) => quote! {
                                #name: self.#name.lerp(other.#name, lerp::num_traits::cast::<_, #ty>(t).expect("casting any Float to #ty should be safe"))
                            },
                            LerpType::Generic => {
                                quote! {
                                    #name: self.#name.lerp(other.#name, t)
                                }
                            },
                            LerpType::Skip => {
                                quote! {
                                    #name: self.#name
                                }
                            }
                        })
                    } else {
                        Err(Error::new(
                            f.ident.span(),
                            "All fields must be named in the struct",
                        ))
                    }
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(quote! {
                #[automatically_derived]
                impl<F: ::lerp::num_traits::Float> Lerp<F> for #name {
                    fn lerp(self, other: Self, t: F) -> Self {
                        Self {
                            #(#fields),*
                        }
                    }
                }
            })
        }
        Fields::Unnamed(fields) => {
            let fields =  fields.unnamed.iter()
           .enumerate()
            .map(|(i, f)| {
                let Field {
                    attrs,
                    ty,
                    ..
                } = f;

                let lerp_type = get_lerp_type(ty, attrs)?;

                Ok(match lerp_type {
                    LerpType::Type(ty) => {
                        let name = Index::from(i);

                        quote! {
                            #name: self.#name.lerp(other.#name, lerp::num_traits::cast::<_, #ty>(t).expect("casting any Float to #ty should be safe"))
                        }
                    },
                    LerpType::Generic => {
                        let name = Index::from(i);

                        quote! {
                            #name: self.#name.lerp(other.#name, t)
                        }
                    },
                    LerpType::Skip => {
                        let name = Index::from(i);

                        quote! {
                            #name: self.#name
                        }
                    }
                })
            })
            .collect::<Result<Vec<_>>>()?;

            Ok(quote! {
                #[automatically_derived]
                impl<F: ::lerp::num_traits::Float> Lerp<F> for #name {
                    fn lerp(self, other: Self, t: F) -> Self {
                        Self {
                            #(#fields),*
                        }
                    }
                }
            })
        }
        _ => Err(Error::new(input.span(), "Struct must have fields")),
    }
}
