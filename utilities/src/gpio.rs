pub struct GpioParser {
    body: proc_macro2::TokenStream,
}

struct KeyValue {
    key: String,
    value: String,
}

impl GpioParser {
    fn expand(&self, ts: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        // let buf = ts.clone().into_iter().collect::<Vec<_>>();
        let ret = proc_macro2::TokenStream::new();
        let buffer=syn::buffer::TokenBuffer::new2(st.body.clone());
        let cursor=buffer.begin();
        while !cursor.eof(){
            if let Some((punct,_,cursor_1)) =cursor.group(proc_macro2::Delimiter::Bracket)  {
                
            }
        }
    }

    fn parse_key_value(&self, c: syn::buffer::Cursor) -> syn::Result<proc_macro2::TokenStream> {
        let mut ret = proc_macro2::TokenStream::new();
        let mut cursor = c;
        if let Some((punct_prefix, cursor_1)) = cursor.punct() {}

        if let Some((group_cur, _, next_cur)) = cursor.group(proc_macro2::Delimiter::Brace) {

        }
    }
}

impl syn::parse::Parse for GpioParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let body_buf;
        syn::braced!(body_buf in input);
        let body: proc_macro2::TokenStream = body_buf.parse()?;
        input.parse();
        Ok(GpioParser { body })
    }
}
