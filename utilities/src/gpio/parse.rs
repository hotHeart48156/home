pub struct GpiosParser {
    pub gpios: Vec<proc_macro2::TokenStream>,
}

const KEYS: [&'static str; 6] = [
    "name",
    "gpio_group",
    "pin",
    "mode",
    "interrput",
    "inter_fun",
];
// fn check_key(key: String) -> (String, bool) {
//     for k in KEYS {
//         if key.eq(&k) {
//             return (key, true);
//         }
//     }
//     return (key, false);
// }

use std::vec;

// use proc_macro2::TokenStream;
use syn::parse::ParseBuffer;

use crate::key_value::KeyValue;

impl GpiosParser {
    pub fn expand(&self, ts: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let buffer = syn::buffer::TokenBuffer::new2(ts.clone());
        let mut cursor = buffer.begin();
        let mut key_values: Vec<KeyValue> = vec![];
        let mut ret: proc_macro2::TokenStream;
        while !cursor.eof() {
            let key_value = KeyValue::new(cursor.clone()).unwrap();
            eprintln!("key:{}--value:{}", key_value.key, key_value.value);
            key_values.push(key_value.clone());
            cursor = key_value.cursor;
          
            // ret.extend(quote::quote! {})
        }

        return proc_macro2::TokenStream::new();
    }
    fn attribute_parse(&self, key_values: Vec<KeyValue>) -> proc_macro2::TokenStream {
        // key_values.into_iter().map(|key_value| {
            
        // }).collect();

        return proc_macro2::TokenStream::new();
    }
}
impl syn::parse::Parse for GpiosParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut all_block: ParseBuffer;
        let mut block: ParseBuffer;
        let mut gpios: Vec<proc_macro2::TokenStream> = vec![];
        syn::bracketed!(all_block in input);
        while !all_block.is_empty() {
            syn::braced!(block in all_block);
            let block_ts = match block.parse::<proc_macro2::TokenStream>() {
                Ok(ts) => ts,
                Err(_) => {
                    return Err(syn::Error::new(
                        block.span(),
                        "curly braces cannot be parsed as tokenstream",
                    ));
                }
            };
            gpios.push(block_ts);
            let _ = match all_block.parse::<syn::Token!(,)>() {
                Ok(comma) => match all_block.is_empty() {
                    true => {
                        return Err(syn::Error::new(
                            block.span(),
                            "There are no curly braces behind, no parentheses can be added",
                        ));
                    }
                    false => comma,
                },
                Err(_) => syn::token::Comma::default(),
            };
        }

        return Ok(GpiosParser { gpios });
    }
}
