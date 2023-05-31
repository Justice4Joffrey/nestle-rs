use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{
    parse2, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Expr, ExprLit, Fields,
    FieldsNamed, FieldsUnnamed, Lit, Type, Variant,
};

use super::parsed_attrs::DeserializeParsedAttrs;
use crate::macros::attrs::{get_nestle_attrs, parse_ident_to_width};

fn parse_enum_discriminant(variant: &Variant) -> syn::Result<i64> {
    match &variant.discriminant {
        Some((_, expr)) => {
            match expr {
                Expr::Lit(ExprLit {
                    lit: Lit::Int(lit_int),
                    ..
                }) => Ok(lit_int.base10_parse::<i64>()?),
                Expr::Unary(expr) => {
                    match expr.op {
                        syn::UnOp::Neg(_) => {
                            let expr = match *expr.expr.clone() {
                                Expr::Lit(ExprLit {
                                    lit: Lit::Int(lit_int),
                                    ..
                                }) => lit_int.base10_parse::<i64>()?,
                                _ => {
                                    Err(syn::Error::new(
                                        variant.span(),
                                        "Discriminant must be an integer.",
                                    ))?
                                }
                            };
                            Ok(-expr)
                        }
                        _ => {
                            Err(syn::Error::new(
                                variant.span(),
                                "Preceding unary operator may only be `-`.",
                            ))
                        }
                    }
                }
                _ => {
                    Err(syn::Error::new(
                        variant.span(),
                        "Discriminant must be an integer.",
                    ))
                }
            }
        }
        _ => Err(syn::Error::new(variant.span(), "Must have a discriminant.")),
    }
}

fn enum_parts(
    data: DataEnum,
    parsed_attrs: DeserializeParsedAttrs,
) -> (u8, TokenStream, TokenStream) {
    let has_data = !data
        .variants
        .first()
        .expect("Enum must have at least one variant.")
        .fields
        .is_empty();
    let repr = parsed_attrs
        .repr
        .expect("Enums must have a `repr` attribute.");
    let (width, self_width, variants, encode_variants) = if has_data {
        let width: u8 = parsed_attrs
            .width
            .expect("Data enums must have a `width` attribute.")
            .base10_parse()
            .expect("Width must be a `u8`.");
        let self_width = parse_ident_to_width(&repr).unwrap();
        let mut variants = Vec::new();
        let mut encode_variants = Vec::new();
        data.variants.into_iter().for_each(|variant| {
            let variant_value = parse_enum_discriminant(&variant).unwrap();
            let variant_name = variant.ident;
            if variant.fields.is_empty() {
                // todo: span
                panic!(
                    "Must provide tuple style data for all variants (check \"{}\").",
                    variant_name
                );
            }
            let inner_ty = match variant.fields {
                Fields::Named(_) => panic!("Must provide tuple style data for all variants."),
                Fields::Unnamed(unnamed) => {
                    let mut iter = unnamed.unnamed.into_iter();
                    let field = iter.next().expect("Missing data");
                    if iter.next().is_some() {
                        panic!("Must provide a single tuple style data for all variants.");
                    }
                    field.ty

                }
                Fields::Unit => panic!("Must provide tuple style data for all variants."),
            };
            variants.push(quote! {
                    #variant_value => {
                        Self::#variant_name(<#inner_ty as ::nestle::core::Nestle>::decode_from(id, offset)?)
                    }
                });
            encode_variants.push(quote! {
                    Self::#variant_name(inner) => <#inner_ty as ::nestle::core::Nestle>::encode_into(inner, id, offset + #self_width)?,
            });
        });
        (width, self_width, variants, {
            quote! {
                match self {
                    #(#encode_variants)*
                }
            }
        })
    } else {
        if parsed_attrs.width.is_some() {
            panic!("Can't specify width for simple enums. Use `repr` attribute instead.");
        };
        let width = parse_ident_to_width(&repr).unwrap();
        (
            width,
            width,
            data.variants
                .into_iter()
                .map(|variant| {
                    let variant_value = parse_enum_discriminant(&variant).unwrap();
                    let variant_name = variant.ident;
                    if !variant.fields.is_empty() {
                        panic!(
                            "Enum variant \"{}\" must not have any fields.",
                            variant_name
                        );
                    }
                    quote! {
                        #variant_value => Self::#variant_name,
                    }
                })
                .collect::<Vec<_>>(),
            TokenStream::new(),
        )
    };
    (
        width,
        quote! {
            let shift = 64 - offset - #self_width;
            *id |= ((unsafe { *<*const _>::from(self).cast::<#repr>() } as i64 as u64) << shift) as i64;
            #encode_variants
            Ok(())
        },
        quote! {
            let shift = 64 - *offset - #self_width;
            let shifted = id >> shift;
            let res = {
                let (res, overflow) = 1_i64.overflowing_shl(#self_width as u32);
                if overflow { 0 } else { res }
            };
            let value = (shifted & (res - 1)) as #repr as i64;
            *offset += #self_width;
            Ok(match value {
                #(#variants)*
                _ => Err(::nestle::core::error::DecodeError::NotFound { typ: Self::TYPE_NAME, disc: value})?
            })
        },
    )
}
fn increment_offset(typ: &Type) -> TokenStream {
    quote! {
        let offset = offset + <#typ as ::nestle::core::Nestle>::WIDTH;
    }
}

fn struct_parts(
    data: DataStruct,
    parsed_attrs: DeserializeParsedAttrs,
) -> (u8, TokenStream, TokenStream) {
    let width = if let Some(width) = parsed_attrs.width {
        width.base10_parse::<u8>().unwrap()
    } else {
        panic!("Structs must have a `width` attribute.")
    };
    let (encode, decode) = match data.fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            let mut fields_decode = Vec::new();
            let mut fields_decode_build = Vec::new();
            let mut fields_encode = Vec::new();
            named.into_iter().for_each(|field| {
                let field_name = field.ident;
                let field_type = field.ty;
                let increment = increment_offset(&field_type);
                fields_decode.push(quote! {
                    let #field_name = ::nestle::core::Nestle::decode_from(id, offset)?;
                });
                fields_decode_build.push(quote! {
                    #field_name,
                });
                fields_encode.push(quote! {
                    <#field_type as ::nestle::core::Nestle>::encode_into(&self.#field_name, id, offset)?;
                    #increment
                });
            });
            (
                quote! {
                    #(#fields_encode)*
                    Ok(())
                },
                quote! {
                    #(#fields_decode)*
                    Ok(Self {
                        #(#fields_decode_build)*
                    })
                },
            )
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let mut fields_decode = Vec::new();
            let mut fields_decode_build = Vec::new();
            let mut fields_encode = Vec::new();
            unnamed.into_iter().enumerate().for_each(|(idx, field)| {
                // TODO: this is very similar to the named case, bar the
                //  field names and the accessor
                let field_name = Ident::new(format!("_{}", idx).as_str(), Span::call_site());
                let field_type = field.ty;
                let increment = increment_offset(&field_type);
                fields_decode.push(quote! {
                    let #field_name = ::nestle::core::Nestle::decode_from(id, offset)?;
                });
                fields_decode_build.push(quote! {
                    #field_name,
                });
                fields_encode.push(quote! {
                    <#field_type as ::nestle::core::Nestle>::encode_into(&#field_name, id, offset)?;
                    #increment
                });
            });
            (
                quote! {
                    let Self(#(#fields_decode_build)*) = self;
                    #(#fields_encode)*
                    Ok(())
                },
                quote! {
                    #(#fields_decode)*
                    Ok(Self (
                        #(#fields_decode_build)*
                    ))
                },
            )
        }
        Fields::Unit => panic!("Structs must have at least one field."),
    };

    (width, encode, decode)
}

pub fn derive_nestle(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        ident,
        data,
        generics,
        ..
    } = parse2::<DeriveInput>(input).unwrap();
    let name = ident.to_string();
    let parsed_attrs = get_nestle_attrs(&attrs);
    let parsed_attrs: DeserializeParsedAttrs = parsed_attrs.try_into().unwrap();

    let (width, encode, decode) = match data {
        Data::Enum(data) => enum_parts(data, parsed_attrs),
        Data::Struct(data) => struct_parts(data, parsed_attrs),
        _ => panic!("This macro can only be derived for enums."),
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::nestle::core::Nestle for #ident #ty_generics #where_clause {
            const WIDTH: u8 = #width;
            const TYPE_NAME: &'static str = #name;
            fn decode_from(id: i64, offset: &mut u8) -> ::std::result::Result<Self, ::nestle::core::error::DecodeError> {
                ::nestle::core::check_i64_width(Self::TYPE_NAME, Self::WIDTH, *offset)?;
                #decode
            }
            fn encode_into(&self, id: &mut i64, offset: u8) -> ::std::result::Result<(), ::nestle::core::error::EncodeError> {
                ::nestle::core::check_i64_width(Self::TYPE_NAME, Self::WIDTH, offset)?;
                #encode
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use syn::parse_quote;

    use super::*;
    use crate::macros::helpers::format_source;

    #[test]
    fn test_derive_simple_enum() {
        let input: TokenStream = parse_quote! {
            #[repr(i16)]
            pub enum Foo {
                Foo = 1,
                Bar = 2,
                Baz = 3,
            }
        };
        assert_snapshot!(
            "simple_enum",
            format_source(derive_nestle(input).to_string().as_str())
        );
    }

    #[test]
    fn test_derive_data_enum() {
        let input: TokenStream = parse_quote! {
            #[nestle(width = 32)]
            #[repr(i16)]
            pub enum Foo {
                Foo(Quux) = 1,
                Bar(i8) = 2,
                Baz(i16) = 3,
            }
        };
        assert_snapshot!(
            "data_enum",
            format_source(derive_nestle(input).to_string().as_str())
        );
    }

    #[test]
    fn test_derive_named_struct() {
        let input: TokenStream = parse_quote! {
            #[nestle(width = 32)]
            pub struct MyStruct {
                field_a: i16,
                field_b: MyEnum
            }
        };
        assert_snapshot!(
            "named_struct",
            format_source(derive_nestle(input).to_string().as_str())
        );
    }

    #[test]
    fn test_derive_tuple_struct() {
        let input: TokenStream = parse_quote! {
            #[nestle(width = 40)]
            pub struct MyStruct (
                MyEnum,
                i16,
                i8
            );
        };
        assert_snapshot!(
            "tuple_struct",
            format_source(derive_nestle(input).to_string().as_str())
        );
    }

    #[test]
    #[should_panic]
    fn test_deserialize_simple_enum_no_repr() {
        let input: TokenStream = parse_quote! {
            #[nestle(width = 16)]
            pub enum Foo {
                Foo = 1,
                Bar = 2,
                Baz = 3,
            }
        };
        derive_nestle(input);
    }

    #[test]
    #[should_panic]
    fn test_simple_enum_supplied_width() {
        let input: TokenStream = parse_quote! {
            #[nestle(width = 16)]
            #[repr(i16)]
            pub enum Foo {
                Foo = 1,
                Bar = 2,
                Baz = 3,
            }
        };
        derive_nestle(input);
    }

    #[test]
    #[should_panic]
    fn test_deserialize_data_enum_no_width() {
        let input: TokenStream = parse_quote! {
            #[repr(i16)]
            pub enum Foo {
                Foo(Quux) = 1,
                Bar(i8) = 2,
                Baz(i16) = 3,
            }
        };
        derive_nestle(input);
    }

    #[test]
    #[should_panic]
    fn test_deserialize_data_enum_no_repr() {
        let input: TokenStream = parse_quote! {
            #[nestle(width = 32)]
            pub enum Foo {
                Foo(Quux) = 1,
                Bar(i8) = 2,
                Baz(i16) = 3,
            }
        };
        derive_nestle(input);
    }

    #[test]
    #[should_panic]
    fn test_deserialize_missing_discriminant() {
        let input = parse_quote! {
            #[repr(i16)]
            pub enum Foo {
                Foo = 1,
                Bar = 2,
                Baz,
            }
        };
        derive_nestle(input);
    }

    #[test]
    #[should_panic]
    fn test_deserialize_empty_enum() {
        let input: TokenStream = parse_quote! {
            #[repr(i16)]
            pub enum Foo {}
        };
        derive_nestle(input);
    }

    #[test]
    #[should_panic]
    fn test_deserialize_mixed_enum() {
        let input: TokenStream = parse_quote! {
            #[repr(i16)]
            #[nestle(descendents = i8)]
            pub enum Foo {
                Foo = 1,
                Bar = 2,
                Baz(Quux) = 3,
            }
        };
        derive_nestle(input);
    }

    #[test]
    #[should_panic]
    fn test_deserialize_mixed_data_enum() {
        let input: TokenStream = parse_quote! {
            #[repr(i16)]
            #[nestle(descendents = i8)]
            pub enum Foo {
                Foo(Quux) = 1,
                Bar = 2,
                Baz = 3,
            }
        };
        derive_nestle(input);
    }
}
