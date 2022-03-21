use proc_macro::TokenStream;
mod  gpio;
#[proc_macro_attribute]
pub fn config(args:TokenStream,input:TokenStream)-> TokenStream {

    // eprintln!("{:#?}",input);
    // eprintln!("{:#?}",args);
    TokenStream::new()
}
