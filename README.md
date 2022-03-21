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
mod searil
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
fn searil(input:tokenstream)->tokenstream{
//todo 生成静态的usart变量，初始化时钟变量的函数
      searil_expand()
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
