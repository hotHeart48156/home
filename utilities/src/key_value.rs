use proc_macro2::Punct;
use syn::buffer::Cursor;

#[derive(Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: syn::Ident,
}
impl KeyValue {
    pub fn new(cursor: Cursor) -> syn::Result<(Self, Cursor)> {
        let (key_ident, key_cursor) = match cursor.ident() {
            Some(key) => key,
            None => {
                let (_value, _cursor) = match cursor.literal() {
                    Some(value) => {
                        if value.0.to_string().parse::<u32>().is_ok() {
                            let ident_number = format!("ident_{}", value.0.to_string());
                            (
                                syn::Ident::new(ident_number.as_str(), value.1.span()),
                                value.1,
                            )
                        } else {
                            (
                                syn::Ident::new(
                                    value.0.to_string().trim_matches(&['\"', '\''] as &[_]),
                                    value.1.span(),
                                ),
                                value.1,
                            )
                        }
                    }
                    None => {
                        return Err(syn::Error::new(cursor.span(), "expect ident"));
                    }
                };
                (_value, _cursor)
            }
        };

        let (_colon_punct, colon_cursor) = match key_cursor.punct() {
            Some(colon) => match colon.0.as_char() {
                ':' => colon,
                _ => {
                    return Err(syn::Error::new(
                        key_cursor.span(),
                        "must be separated by colons;",
                    ))
                }
            },
            None => {
                return Err(syn::Error::new(
                    key_cursor.span(),
                    "must be separated by colons;\n:",
                ));
            }
        };
        let (value_ident, value_cursor) = match colon_cursor.ident() {
            Some(value) => (
                syn::Ident::new(value.0.to_string().trim_matches('\"'), value.1.span()),
                value.1,
            ),
            None => {
                let (_value, _cursor) = match colon_cursor.literal() {
                    Some(value) => {
                        if value.0.to_string().parse::<u32>().is_ok() {
                            let ident_number = format!("ident_{}", value.0.to_string());
                            (
                                syn::Ident::new(ident_number.as_str(), value.1.span()),
                                value.1,
                            )
                        } else {
                            (
                                syn::Ident::new(
                                    value.0.to_string().trim_matches(&['\"', '\''] as &[_]),
                                    value.1.span(),
                                ),
                                value.1,
                            )
                        }
                    }
                    None => {
                        return Err(syn::Error::new(colon_cursor.span(), "must have value"));
                    }
                };
                (_value, _cursor)
            }
        };
        let (_comma_punct, comma_cursor) = match value_cursor.punct() {
            Some(common) => match common.0.as_char() {
                ',' => common,
                _ => return Err(syn::Error::new(cursor.span(), "must have value")),
            },
            None => (Punct::new(',', proc_macro2::Spacing::Alone), value_cursor),
        };
        Ok((
            KeyValue {
                key: key_ident.to_string(),
                value: value_ident,
            },
            comma_cursor,
        ))
    }
}
