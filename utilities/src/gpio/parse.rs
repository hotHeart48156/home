pub struct GpiosParser {
    pub gpios: Vec<proc_macro2::TokenStream>,
}
use crate::key_value::KeyValue;
use std::vec;
use syn::parse::ParseBuffer;
// use std::vec::IntoIter;
// use std::slice::Iter;
impl GpiosParser {
    pub fn expand(&self, ts: &proc_macro2::TokenStream) -> syn::Result<Vec<KeyValue>> {
        let buffer = syn::buffer::TokenBuffer::new2(ts.clone());
        let mut cursor = buffer.begin();
        let mut key_values: Vec<KeyValue> = vec![];
        // let mut ret: proc_macro2::TokenStream;
        while !cursor.eof() {
            let key_value = match KeyValue::new(cursor.to_owned()) {
                Ok(key_value) => key_value,
                Err(_) => return Err(syn::Error::new(cursor.span(), "parse error")),
            };
            // eprintln!("key:{}--value:{}", key_value.key, key_value.value);
            key_values.push(key_value.0.clone());
            cursor = key_value.1;
        }
        Ok(key_values)
    }
    // fn attribute_parse(&self, key_values: Vec<KeyValue>) -> proc_macro2::TokenStream {
    //     return proc_macro2::TokenStream::new();
    // }
}
impl syn::parse::Parse for GpiosParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let all_block: ParseBuffer;
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
