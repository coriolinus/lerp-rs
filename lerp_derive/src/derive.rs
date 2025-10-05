use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Error, Field, Fields, Index, ItemStruct, Path, Result, Token, Type, TypeGroup,
    TypeParen, TypePath,
};

#[derive(Default)]
struct LerpAttributes {
    skip: bool,
    type_override: Option<Path>,
}

impl Parse for LerpAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let inputs = Punctuated::<Path, Token![,]>::parse_terminated(input)?;

        let mut skip = false;
        let mut type_override = None;

        for path in inputs {
            if path.is_ident("skip") || path.is_ident("ignore") {
                if skip {
                    return Err(Error::new(path.span(), "duplicate skip statement"));
                }

                skip = true;
            } else {
                if type_override.is_some() {
                    return Err(Error::new(path.span(), "duplicate occurrence of lerp type"));
                }

                type_override = Some(path);
            }
        }

        Ok(Self {
            skip,
            type_override,
        })
    }
}

fn lerp_field(name: &dyn ToTokens, ty: &Type, attrs: &Vec<Attribute>) -> syn::Result<TokenStream> {
    let attr = attrs
        .iter()
        .filter(|attr| attr.path.is_ident("lerp"))
        .collect::<Vec<_>>();

    let attr: LerpAttributes = match &attr[..] {
        [] => Ok(Default::default()),
        [attr] => attr.parse_args(),
        [_, overflow, ..] => Err(Error::new(
            overflow.span(),
            "found duplicate attribute on field, consolidate the attributes into one",
        )),
    }?;

    if attr.skip {
        Ok(quote! {
            #name: self.#name
        })
    } else {
        match ty {
            Type::Path(TypePath { path, .. }) => Ok({
                let path = attr.type_override.as_ref().unwrap_or(path);

                if path.is_ident("f64") || path.is_ident("f32") || attr.type_override.is_some() {
                    quote! {
                        #name: self.#name.lerp(other.#name, cast::<_, #path>(t).unwrap_or_else(|| panic!("casting any Float to {} should be safe", stringify!(#path))))
                    }
                } else {
                    quote! {
                        #name: self.#name.lerp(other.#name, t)
                    }
                }
            }),

            // Recursively descend through groups
            Type::Group(TypeGroup { elem, .. }) | Type::Paren(TypeParen { elem, .. }) => {
                lerp_field(name, elem, attrs)
            }

            // TODO: Support the types outlined in issue #6 <https://github.com/coriolinus/lerp-rs/issues/6>
            _ => Err(Error::new(ty.span(), "Unsupported type.\nSee issue #6 <https://github.com/coriolinus/lerp-rs/issues/6> for more information")),
        }
    }
}

pub fn lerp_derive_internal(input: &ItemStruct) -> Result<TokenStream> {
    let name = &input.ident;

    let fields = match &input.fields {
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
                        lerp_field(&name, ty, attrs)
                    } else {
                        Err(Error::new(
                            f.ident.span(),
                            "All fields must be named in the struct",
                        ))
                    }
                })
                .collect::<syn::Result<Vec<_>>>()?;

            Ok(fields)
        }
        Fields::Unnamed(fields) => {
            let fields = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| {
                    let name = Index::from(i);

                    lerp_field(&name, &f.ty, &f.attrs)
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(fields)
        }
        _ => Err(Error::new(input.span(), "Struct must have fields")),
    }?;

    Ok(quote! {
        #[automatically_derived]
        impl<F: ::lerp::num_traits::Float> Lerp<F> for #name {
            fn lerp(self, other: Self, t: F) -> Self {
                #[allow(unused)]
                use lerp::num_traits::cast;

                Self {
                    #(#fields),*
                }
            }
        }
    })
}
