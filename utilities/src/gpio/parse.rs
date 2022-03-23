pub struct GpioParser {
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

use proc_macro2::TokenStream;
use syn::parse::ParseBuffer;

impl GpioParser {
    pub fn expand(&self, ts: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        // let buf = ts.clone().into_iter().collect::<Vec<_>>();
        let ret = proc_macro2::TokenStream::new();
        let buffer = syn::buffer::TokenBuffer::new2(ts.clone());
        let cursor = buffer.begin();
        if let Some((bracke_start_next, _, bracke_end_next)) =
            cursor.group(proc_macro2::Delimiter::Parenthesis)
        {
            let (a, b) = bracke_start_next.ident().unwrap();
            // self.parse_groups(bracke_start_next)
            eprintln!("aacmsklcmsdl{:#?}", a.to_string());
        }
        // eprintln!("{:#?}",ts);
        ret
    }

    // fn parse_groups(&self, cursor: syn::buffer::Cursor) -> syn::Result<proc_macro2::TokenStream> {
    //     while let Some(brace_start_next, _, brace_end_next) =
    //         cursor.group(proc_macro2::Delimiter::Brace)
    //     {
    //         self.parse_group(brace_start_next, brace_end_next.clone());
    //         cursor = brace_end_next;
    //     }
    // }
}
impl syn::parse::Parse for GpioParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut all_block: ParseBuffer;
        let mut first_block: ParseBuffer;
        let mut secode_block: ParseBuffer;
        syn::bracketed!(all_block in input);
        syn::braced!(first_block in all_block);
        eprintln!("first block{:#?}", all_block);
        let com = all_block.parse::<syn::Token!(,)>()?;
        eprintln!("common{:#?}", com);
        syn::braced!(secode_block in all_block);
        eprintln!("sencode block{:#?}", secode_block);
        return Ok(GpioParser {
            gpios: vec![
                first_block.parse::<proc_macro2::TokenStream>()?,
                secode_block.parse::<proc_macro2::TokenStream>()?,
            ],
        });
    }
}
