use std::{str::FromStr, vec};

use proc_macro2::{Literal, Punct};

pub struct KeyValue {
    pub key: String,
    pub value: String,
}
impl syn::parse::Parse for KeyValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let cursor = input.cursor();
        let (key_ident, key_cursor) = match cursor.ident() {
            Some(key) => key,
            None => {
                return Err(syn::Error::new(cursor.span(), "expect ident"));
            }
        };
        // eprintln!("ident{:#?}", id.0.to_string());
        let (colon_punct, colon_cursor) = match key_cursor.punct() {
            Some(colon) => match colon.0.as_char() {
                ':' => colon,
                _ => {
                    return Err(syn::Error::new(
                        cursor.span(),
                        "must be separated by colons;",
                    ))
                }
            },
            None => {
                return Err(syn::Error::new(
                    cursor.span(),
                    "must be separated by colons;\n:",
                ));
            }
        };
        // eprintln!("punct{:#?}", punct.0.to_string());
        let (value_literal, value_cursor) = match colon_cursor.literal() {
            Some(value) => value,
            None => {
                // match colon_cursor.ident() {
                //     Some(value) => (Literal::from_str(value.0.to_string().as_str()), value.1),
                //     None => (Literal::from_str(""), colon_cursor),
                // }
                return Err(syn::Error::new(cursor.span(), "must have value"));
            }
        };
        // eprintln!("liter{:#?}", liter.0.to_string());
        let (comma_punct, comma_cursor) = match value_cursor.punct() {
            Some(common) => match common.0.as_char() {
                ',' => common,
                _ => return Err(syn::Error::new(cursor.span(), "must have value")),
            },
            None => (Punct::new(',', proc_macro2::Spacing::Alone), value_cursor),
        };
        Ok(KeyValue {
            key: key_ident.to_string(),
            value: value_literal.to_string(),
        })
    }
}
