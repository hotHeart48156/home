use proc_macro::TokenStream;
pub mod  gpio;
#[proc_macro_attribute]
pub fn config(args:TokenStream,input:TokenStream)-> TokenStream {

    eprintln!("{:#?}",input);
    eprintln!("{:#?}",args);
    TokenStream::new()
}
