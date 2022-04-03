use proc_macro::TokenStream;
use quote::ToTokens;
mod gpio;
mod init;
mod key_value;
mod serial;
mod time;
mod wrap;
#[proc_macro]
pub fn time(input: TokenStream) -> TokenStream {
    let gp = syn::parse_macro_input!(input as time::TimeParse);
    let mut ret = proc_macro2::TokenStream::new();
    let cc = gp.expand();
    ret.extend(cc);
    ret.into()
}
#[proc_macro]
pub fn gpio(input: TokenStream) -> TokenStream {
    let gp = syn::parse_macro_input!(input as gpio::parse::GpiosParser);
    let mut ret = proc_macro2::TokenStream::new();
    let _c: Vec<&str> = gp
        .gpios
        .clone()
        .into_iter()
        .map(|ts| {
            let sigle_gpio_key_value = match gp.expand(&ts.clone()) {
                Ok(ok) => ok,
                Err(_) => {
                    return "";
                }
            };
            let quote = match gpio::convert::convert_gpio_struct_to_quote(sigle_gpio_key_value) {
                Ok(ok) => ok,
                Err(_) => {
                    return "";
                }
            };
            ret.extend(quote);
            ""
        })
        .collect();
    ret.into()
}
#[proc_macro]
pub fn serial(input: TokenStream) -> TokenStream {
    let gp = syn::parse_macro_input!(input as serial::parse::SerialParser);
    let mut ret = proc_macro2::TokenStream::new();
    let _c: Vec<&str> = gp
        .serials
        .clone()
        .into_iter()
        .map(|ts| {
            let sigle_gpio_key_value = match gp.expand(&ts.clone()) {
                Ok(ok) => ok,
                Err(_) => {
                    return "";
                }
            };
            let quote = match serial::convert::convert_serial_struct_to_quote(sigle_gpio_key_value)
            {
                Ok(ok) => ok,
                Err(_) => {
                    return "";
                }
            };
            ret.extend(quote);
            ""
        })
        .collect();
    ret.into()
}
#[proc_macro_attribute]
pub fn init(atts: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(atts as syn::AttributeArgs);
    let att = args[0].to_token_stream();
    let att_tk: proc_macro::TokenStream = att.into();
    let att_meta = syn::parse_macro_input!(att_tk as syn::Meta);
    let init_functions_semi_vec = match init::expand_attr(&att_meta) {
        Ok(ok) => ok,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };
    let mut semis: Vec<syn::Stmt> = vec![];
    let _ = init_functions_semi_vec
        .into_iter()
        .map(|punct| {
            let semi = init::new_stmi(punct).unwrap();
            semis.push(semi);
            0
        })
        .collect::<Vec<u32>>();
    let mut function = syn::parse_macro_input!(input as syn::ItemFn);

    function.block.stmts.extend(semis);
    function.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn wrap(atts: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(atts as syn::AttributeArgs);
    let att = args[0].to_token_stream();
    let att_tk: proc_macro::TokenStream = att.into();
    let att_meta = syn::parse_macro_input!(att_tk as syn::Meta);
    let init_functions_semi_vec = match wrap::parse_attr::parse_attr(&att_meta) {
        Ok(ok) => ok,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };
    let mut function = syn::parse_macro_input!(input as syn::ItemFn);
    // function.block;
    // eprintln!("{:#?}",function);
    let before_stmt = function.block.stmts.clone();
    let stmt_semi = wrap::function_gen::gen_free_stmt_stmi(init_functions_semi_vec, before_stmt);
    // eprintln!("{:#?}",input);

    function.block.stmts = vec![stmt_semi];
    // let ret = proc_macro2::TokenStream::new();
    // ret.into()
    function.into_token_stream().into()
}
