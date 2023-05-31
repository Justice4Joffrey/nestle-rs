use syn::{Ident, LitInt};

use crate::macros::attrs::{duplicate_attribute, NestleAttr};

pub struct DeserializeParsedAttrs {
    pub repr: Option<Ident>,
    pub width: Option<LitInt>,
}

impl TryFrom<Vec<NestleAttr>> for DeserializeParsedAttrs {
    type Error = syn::Error;

    fn try_from(attrs: Vec<NestleAttr>) -> Result<Self, Self::Error> {
        let mut this = DeserializeParsedAttrs {
            repr: None,
            width: None,
        };
        for v in attrs.into_iter() {
            match v {
                NestleAttr::Width(s, _, id) => {
                    if this.width.is_none() {
                        this.width = Some(id);
                    } else {
                        return Err(duplicate_attribute(s));
                    }
                }
                NestleAttr::Repr(s, id) => {
                    if this.repr.is_none() {
                        this.repr = Some(id);
                    } else {
                        return Err(duplicate_attribute(s));
                    }
                }
            }
        }
        Ok(this)
    }
}
