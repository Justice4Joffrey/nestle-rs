use proc_macro2::{Ident, Span};
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    spanned::Spanned,
    token,
    token::Comma,
    Attribute, LitInt, Meta,
};

pub const WIDTH: &str = "width";

pub fn unexpected_attribute(attr: &str, span: Span) -> syn::Error {
    syn::Error::new(span, format!("Unexpected attribute \"{:?}\"", attr))
}

pub fn duplicate_attribute(span: Span) -> syn::Error {
    syn::Error::new(span, "Attribute specified more than once")
}

pub enum NestleAttr {
    Width(Span, token::Eq, LitInt),
    Repr(Span, Ident),
}

impl Parse for NestleAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        match ident.to_string().as_str() {
            WIDTH => Ok(Self::Width(ident.span(), input.parse()?, input.parse()?)),
            unknown => Err(unexpected_attribute(unknown, ident.span())),
        }
    }
}

/// The attribute outer token.
const NESTLE: &str = "nestle";

/// Extracts an attribute token containing multiple attribute declarations.
///
/// e.g.
///
/// #[nestle(a = 1, b)]
/// #[nestle(c)]
///
/// becomes
///
/// vec![a = 1, b, c]
struct NestleAttrs<T> {
    attrs: Punctuated<T, Comma>,
}

impl<T: Parse> Parse for NestleAttrs<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.parse_terminated(T::parse, Comma)?,
        })
    }
}

fn get_nestle_attrs_res(attrs: &[Attribute]) -> syn::Result<Vec<NestleAttr>> {
    let mut res = Vec::new();
    for a in attrs {
        if a.path().is_ident(NESTLE) {
            if let Meta::List(l) = &a.meta {
                let nes_attrs = parse2::<NestleAttrs<NestleAttr>>(l.tokens.clone())?;
                nes_attrs.attrs.into_iter().for_each(|a| res.push(a))
            } else {
                return Err(syn::Error::new_spanned(a, "invalid attribute format"));
            }
        } else if a.path().is_ident("repr") {
            res.push(NestleAttr::Repr(a.span(), a.parse_args()?));
        }
    }
    Ok(res)
}

pub fn get_nestle_attrs(attrs: &[Attribute]) -> Vec<NestleAttr> {
    get_nestle_attrs_res(attrs)
        .expect("attributes must be in the form \"#[nestle(.. = .., ..)]\" or \"#[repr(..)]\"")
}

pub fn parse_ident_to_width(ident: &Ident) -> syn::Result<u8> {
    match ident.to_string().as_str() {
        "i8" => Ok(8),
        "i16" => Ok(16),
        "i32" => Ok(32),
        "i64" => Ok(64),
        _ => {
            Err(syn::Error::new(
                ident.span(),
                format!("`{}` is not a supported signed integer type", ident),
            ))
        }
    }
}
