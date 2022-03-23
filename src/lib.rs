#![no_std]
pub mod bump_alloc;
pub mod utiliti;
// pub mod interrupt;
extern crate alloc;
extern crate cortex_m;
use alloc::{string::String, vec::Vec};
// use core::alloc::GlobalAlloc;
// use core::ptr;
// use core::cell::UnsafeCell;
// use cortex_m::interrupt;
// use cortex_m::asm;
// use core::alloc::Layout;
use utilities::gpio;
// enum GpioMode {
//     PushPull
// }
// struct GpioConf{
//     name:String,
//     gpio_group:String,
//     gpio_pin:u8,
//     mode:GpioMode
// }
// struct SerialConf{
//     serial:String,
//     gpio_tx:String,
//     gpio_rx:String,
//     mode:GpioMode,
//     baud_rate:u64
// }
// struct DeviceConf {
//     system_clock: u32,

//     gpios: Option<Vec<GpioConf>>,
// }

gpio!(

    [
        {
            name:"led",gpio_group:"gpioe",pin:3,
            mode:"pushpull",interrput:"EXIT3",inter_fun:"handle_exit"
        },
        {
            name:"red",gpio_group:"gpioec",pin:5,
            mode:"pushpull",interrput:"EXIT3",inter_fun:"handle_exit_second"
        }
    ]

);
// gpio!(
//   {
//       [],[]
//   }

// );

// gpio!(
//     [
//         {cslm:"cs",}
//     ]
// );
