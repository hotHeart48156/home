use proc_macro::TokenStream;
mod gpio;
mod init;
mod serial;
mod time;
#[proc_macro]
pub fn time(input: TokenStream) -> TokenStream {
    //todo 生成静态的时钟变量，总线变量和初始化时钟变量的函数
    //实现函数在
    time_expand()
}
#[proc_macro]
pub fn gpio(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as gpio::GpioParser);
    let mut ret = proc_macro2::TokenStream::new();
    
    let buffer = syn::buffer::TokenBuffer::new2(st.body.clone());//tokenstream 生成tokenbuffer
    return ret.into()
}
#[proc_macro]
pub fn serial(input: TokenStream) -> TokenStream {
    //todo 生成静态的usart变量，初始化时钟变量的函数
    
    serial_expand()
}
#[proc_macro_attribute]
pub fn init() {}
