svd下载：https://github.com/tinygo-org/stm32-svd/tree/main/svd
stm cfg下载：https://github.com/openocd-org/openocd/tree/master/tcl/target
stlink/ulink/jlink cfg下载：https://github.com/openrisc/openOCD/tree/master/tcl/interface

2022-3-21方案：
使用macro_rule !或者使用proc_macro来处理设备初始化，生成相应的初始化函数在main中第一行调用
首先必须初始化time，然后是其他外设或者gpio
初始化总线或者其他设备自动设置相应的全局变量
example:
mod time
mod gpio
mod serial
#[proc_macro]
fn time(input:tokenstream)->tokenstream{
//todo 生成静态的时钟变量，总线变量和初始化时钟变量的函数
//实现函数在
    time_expand()
}
#[proc_macro]
fn gpio(input:tokenstream)->tokenstream{
//todo 生成静态的gpio变量，初始化时钟变量的函数
    gpio_expand()
}
#[proc_macro]
fn serial(input:tokenstream)->tokenstream{
//todo 生成静态的usart变量，初始化时钟变量的函数
      serial_expand()
}
#[proc_macro_attribute]
fn init(){

}
--------------------------------------
#[proc_macro]
fn spi(input:tokenstream)->tokenstream{
//todo 生成静态的usart变量，初始化时钟变量的函数
      searil_expand()
}
usage:
    lib.rs
        time(100);
        gpio(
            [
                {
                    name:"led",gpio_group:"gpioe",pin:3,
                    mode:"pushpull",interrput:"EXIT3",inter_fun:handle_exit
                }
            ]
        );
        searil(
            [
                {
                    name:"led",tx:"wifi_tx",rx:"wifi_rx"
                    baud_rate:115200,interrput:"EXIT3",inter_fun:handle_exit
                }
            ]
        );
    main.rs
  //   use home::time_init;
  //   use home::gpio_init;

     #[entry]
     #[init(time,led,usart)]
     main(){
         // home::time_init();由init完成调用
         // home::led_init()
     }

-------------------------
|proc-macro::TokenStream| proc_macro宏就是要初始的就是stream|不能解析内容|直接返回|
-------------------------

|proc-macro2::TokenStream|生成:proc-macro::TokenStream，|由以下可以生成|可以解析成vec<TokenTree>数组，通过遍历解析，可以解析括号，提取括号里面的内容|不能解析括号的类型|
let buf = ts.clone().into_iter().collect::<Vec<_>>();
let tree_node = &buf[idx];
match tree_node{
    proc_macro2::TokenTree::Group(g) => {
    // 如果是括号包含的内容，我们就要递归处理内部的TokenStream
    let new_stream = self.expand(&g.stream(), n);
}
通过into生成proc-macro::TokenStream
-------------------------

|syn::ParseStream|syn把proc-macro::TokenStream，变为ParseStream|可以解析内容|parse只能解析连续的用空格分开的，不能解析括号|可以用peek查看后面的token，不移动位置|
input: syn::parse::ParseStream
input.parse::<syn::Token!(in)>()?;
let start: syn::LitInt = input.parse()?;

let body_buf;
syn::braced!(body_buf in input);
let body: proc_macro2::TokenStream  = body_buf.parse()?;
-------------------------

|syn::TokenBuffer|和syn::ParseStream差不多,通过begin得到Cursor|通过c来解析用空格分开的字符|
syn::buffer::TokenBuffer::new2(st.body.clone());通过最初的TokenStream生成TokenBuffer
-------------------------
|TokenTree|
|ParseBuffer|ParseStream到proc-macro2::TokenStream不能直接转要通过ParseBuffer|
let body: proc_macro2::TokenStream = body_buf.parse()?;
input.parse();
-------------------------

|cursor|一步一步的解析|可以解析括号的类型|也可以遍历|可以根据返回的结果来跳转|
解析方括号最好把proc-macro::TokenStream转为proc-macro2::TokenStream
解析花括号也是使用proc-macro2::TokenStream，我能匹配到括号，但是不知道括号是什么
解析花括号里面的多个key_value使用cursor，解析单个key_value使用cursor


tokenstream -> tokenbuffer -> cursor