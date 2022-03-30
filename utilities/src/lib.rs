use proc_macro::TokenStream;
mod gpio;
mod init;
mod key_value;
mod serial;
mod time;
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
    eprintln!("{:#?}",ret.to_string());
     ret.into()
}
// #[proc_macro]
// pub fn serial(input: TokenStream) -> TokenStream {
//     //todo 生成静态的usart变量，初始化时钟变量的函数

//     serial_expand()
// }
// #[proc_macro_attribute]
// pub fn init() {}
