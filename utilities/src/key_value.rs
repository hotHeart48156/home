// use std::{str::FromStr, vec};

use std::str::FromStr;

use proc_macro2::{Literal, Punct};
use syn::buffer::Cursor;

#[derive(Clone)]
pub struct KeyValue<'a> {
    pub key: String,
    pub value: String,
    pub cursor: Cursor<'a>,
}
impl<'a> KeyValue<'a> {
    pub fn new(cursor: Cursor<'a>) -> syn::Result<Self> {
        // let cursor = input.cursor();
        let (key_ident, key_cursor) = match cursor.ident() {
            Some(key) => key,
            None => {
                return Err(syn::Error::new(cursor.span(), "expect ident"));
            }
        };

        // eprintln!("ident{:#?}", id.0.to_string());
        let (_colon_punct, colon_cursor) = match key_cursor.punct() {
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
                match colon_cursor.ident() {
                    Some(value) => (Literal::from_str(value.0.to_string().as_str()), value.1),
                    None => (Literal::from_str("s"), colon_cursor),
                };
                return Err(syn::Error::new(cursor.span(), "must have value"));
            }
        };
        // let value_cursor:Cursor<'a>=value_cursor.clone();
        // eprintln!("liter{:#?}", liter.0.to_string());
        let (_comma_punct, comma_cursor) = match value_cursor.punct() {
            Some(common) => match common.0.as_char() {
                ',' => common,
                _ => return Err(syn::Error::new(cursor.span(), "must have value")),
            },
            None => (Punct::new(',', proc_macro2::Spacing::Alone), value_cursor),
        };
        Ok(KeyValue {
            key: key_ident.to_string(),
            value: value_literal.to_string(),
            cursor: comma_cursor,
        })
    }
}
