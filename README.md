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
2022-3月24日方案
gpio：
    先初始化相应的静态变量
    提取相应的变量，到结构体里面
    struct  GpioStruct{
        name:Option<String>,
        mode::Option<String>,
        gpio_group:Option<String>,
        pin:Option<String>,
        interrupt:Option<String>,
        priority:Option<String>,
    }
    struct Mode{
        forward:String,
        mode:String,
        function:String
    }
    const MODE_MAP:map<String,Mode>=[
        { "push_pull":Mode{forward:"Input",mode:"Pull","function":"into_pull_up_input"}},
        { "open_drain":Mode{forward:"Input",mode:"Pull","function":"into_open_drain"}}
    ]
    fn parse(key_values:Vec<KeyValue>)->GpioStruct{
        
    let name:Option<String>=None;
    let gpio_group:Option<String>=None;
    let mode:Option<String>=None;
    let pin:Option<String>=None;
    let interrupt:Option<String>=None;
    let priority:Option<String>=None;

    for key_value in key_values{
        match key_value{
            "name"=>name=key_value.value,
            "gpio_group"=>gpio_group=key_value.value,
            "pin"=>pin=key_value.value,
            "interrupt"=>interrupt=key_value.value,
            "priority"=>priority=key_value.value,
            "mode"=>mode=key_value.value,
        }
    }
    GpioStruct{name,gpio_group,pin,interrupt,priotiry,mode}
    }

    gpio_struct=parse(key_values);
    let name=match gpio_struct.name{
        Some(name)=>name,
        None=>return Err();
    };
    let mode=match gpio_struct.mode{
        Some(mode)=>mode,
        None=>return Err();
    };
    let gpio_group=match gpio_struct.gpio_group{
        Some(gpio_group)=>gpio_group,
        None=>return Err();
    };
    let pin=match gpio_struct.pin{
        Some(pin)=>pin,
        None=>return Err();
    };
    let interrupt=match gpio_struct.interrupt{
        Some(interrupt)=>interrupt,
        None=>"";
    };
    let priority=match gpio_struct.priority{
        Some(priority)=>priority,
        None=>"";
    };
    let ident_mode=MODE_MAP.get(mode);
    let forward=ident_mode.forward;//此处不能直接在quote中用点。
    let mode=ident_mode.mode;
    let static_quote_block=
     quote::quote! {
                  static #name: Mutex<RefCell<Option<PE3<#forward<#mode>>>>> = Mutex::new(RefCell::new(None));
                  }
    let init_quote_function=quote::quote!{
        fn led_init() {
        free(|cs| {
        let _dp = DP.borrow(cs).take().unwrap();
        let _ccdr = CCDR.borrow(cs).take().unwrap();
        let _gpioe = _dp.GPIOE.split(_ccdr.peripheral.GPIOE);
        let mut _led = _gpioe.pe3.into_pull_up_input();
        // _led.make_interrupt_source(&mut syscfg);
        // _led.enable_interrupt(&mut dp.EXTI);
        // _led.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
        name.borrow(cs).replace(Some(_led));
    });
        free(|cs| {
        let mut _cp = CP.borrow(cs).take().unwrap();
        unsafe {
            _cp.NVIC.set_priority(interrupt::#interrupt, #priority);
            NVIC::unmask::<interrupt>(interrupt::#interrupt);
            _cp.NVIC.set_priority(interrupt::#interrupt, #priority);
            NVIC::unmask::<interrupt>(interrupt::#interrupt,);
        }
    })
    let mut ret=static_quote_block;
    let ret.extend(init_quote_function);
    return Ok(ret);
}
    }
------------------------------------------------------------------
#lib.rs
parse(key_value).into;
--------------------------------------------------------------------
2022-3月-25日

parse意味把单个tokenstream解析为多个有意义的tokenstream，expand意味解析一个有意义的tokenstream为
一个vec<keyvalue>
现在要把keyvalue解析为新的代码，convert_map_to_quote
--------------------------------------------------------------------
用python解析表格思路
class attribute{
    name:str
    value:str
    time:str

    getvalue(){
        if(value.0.contain("未"))return 1;else return 0
    }
}

class table{
    attributes:list
}
提取所以属性的时间为一个list，用matlib解析
time_line=[]
name_line=[]
for att in attributes:
    time_lime.add(att.value)
    name_line.add(att.name)
提取所有属性的值
matlib.show()


-----------------------------------------------------------------------------------------------------
链接脚本相关

项目创建之初，新建了.cargo/config.toml文件
  里面有这样一句  "-C","link-arg=-Tlink.x",
  cargo是项目管理器，rustc是真实的编译器，上面的一句话会作用到每一次rustc编译的时候
  https://doc.rust-lang.org/cargo/reference/config.html
  https://doc.rust-lang.org/rustc/codegen-options/index.html
  link-arg制定了链接脚本
  在c语言中由ld来把多个.o文件链接成exe，在rust中是lld
  lld是ld的代替品，所以ld的命令他都是支持的
  ld的-T参数指定的是链接脚本


rust项目有一个build。rs，cargo在编译项目之前会先执行这个build。rs
编译前的准备工作

如果是依赖结构，cargo会先编译依赖项的build。rs,在编译那个项目
https://github.com/rust-embedded/cortex-m/tree/master/cortex-m-rt
我们以依赖了这个项目，所以先编译这个项目的build.rs文件，这个build.rs就是把link.x.in转化为link.x的脚本

link.x文件会引用memory.x文件

链接脚本

如果是同一种cpu链接脚本脚本可以通用
但是不同cpu不能通用。
gcc的编译是把每一个c文件编译成一个.o文件，有多少个c文件，就有多少个.o文件
理论上gcc编译的和rust编译的可以通过ld链接到一起，但是rust在编译的时候会进行代码混淆，可以通过#[no_mangle]来禁止混淆，
rust编译文件的时候，通过mian.rs和lib.rs来自动找到这个项目的所有文件，并且根据codegen-units编译参数制定生成的数量，比方说指定为16个，那么无论有多少个.rs文件，rustc就会16个.o文件。
rustc


memory.x文件
制定了起始地址，和大小
-----------------------------------------------------------------------------------------------------
进程调度

现在keil中调试汇编代码，然后再复制过来

在stm32中有16个寄存器，分别为r0-r15。
其中
r0-r3 储存的是传入的参数，
r4-r11 存放局部变量
r12 暂时寄存器，保存下句指令的地址
r13 sp指针，指向内存地址
r14 lr寄存器,保存了返回地址
r15 pc,程序计数器

汇编函数的编写
当编写汇编函数时，传入的参数自动按照参数的个数自动到r0-r3中
当传入一个参数是，参数在r0,传入两个参数,参数在r0和r1

硬件中断会由硬件保存寄存器
ro-r3
r12和lr并且lr是一个特殊值,可能==0xffff_fff9
当cpu发现lr是一个特殊的值时，它会自动恢复现场

如果我们要调用硬件中断，就要去取到这个特殊的值，并把它给到lr
最终，在执行systick的时候，让他去执行一个我们写好的汇编，这个汇编保存lr的值到r0,再执行systick_handle函数

或者我们调用systick_handle的时候需要传入lr,然后把lr传入到启动程序的汇编里

程序启动关键
StartTask PROC 
    ; r0 保存有任务的栈地址
    ; r1 保存有触发硬件中断的值

    LDMIA R0! , {R4-R11};从栈的开始取出值，保存到r4-r11
    MSP MSP,R0;更新sp，不能用mov
    BX R1;触发硬件中断
    ENDP
在汇编中实现了StartTask需要导出,EXPORT StartTask

切换任务

现在有三个函数
以前执行的函数，程序切换的函数，以后执行的函数
第一步：包保存以前执行的函数
    就是要保存前一个任务的r4-r11，等到我们再次执行的时候，把r4-r11恢复，在更改pc

    在从以前执行的函数，转到程序切换函数时，必须要保证不能有局部变量，否则他会破坏r4-r11。

    调用systick_handle的汇编函数的时候
    先保存r4-r11寄存器和lr寄存器到内存，
    
    再调用程序切换函数，
    调用完之后从sp恢复寄存器
Systick_Hanelder_asm PROC
    STMDB SP! ,{R4-R11};保存以前执行的函数的寄存器
    STMDB SP! ,{LR};保存lr寄存器
    MOV R0 , LR ;相当于给Systick_Handler的第一个参数赋值
    ADD R1,SP ,#4;给SysTick_Handler的第二个参数赋值
    BL SysTick_Handler;转到程序切换函数
    LDMIA SP! ,{R0};把r0赋值给sp
    LDMIA SP! ,{R4-R11};把r4-r11赋值给sp
    BX r0;调用硬件中断

赋值改变之前关中断，改变之后开中断


