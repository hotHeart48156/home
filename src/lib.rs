#![no_std]
pub mod bump_alloc;
pub mod utiliti;
// pub mod interrupt;
extern crate alloc;
extern crate cortex_m;
// use stm32h7xx_hal::pac;
// use core::borrow::Borrow;
// use core::cell::RefCell;
// use cortex_m::interrupt::{free, Mutex};
// use cortex_m::peripheral::NVIC;
// use stm32h7xx_hal::gpio::{Edge, PushPull, Output};
// use stm32h7xx_hal::gpio::{gpioe::PE3, Input, PullUp};
// use stm32h7xx_hal::rng::Rng;
// use utilities::gpio;
// use utilities::time;
// time!(100);
// gpio!(

//     [
//         {
//             name:"led",gpio_group:"gpioe",pin:3,
//             mode:"push_pull",interrput:"EXIT3",inter_fun:"handle_exit"
//         },
//         {
//             name:"red",gpio_group:"gpioec",pin:5,
//             mode:"push_pull",interrput:"EXIT3",inter_fun:"handle_exit_second"
//         },
//         {
//             name:"red",gpio_group:"gpioec",pin:5,
//             mode:"push_pull",interrput:"EXIT3",inter_fun:"handle_exit_second"
//         }
//     ]

// );

// pub static led: cortex_m::interrupt::Mutex<
//     core::cell::RefCell<
//         core::option::Option<
//             stm32h7xx_hal::gpio::gpioe::PE3<
//                 stm32h7xx_hal::gpio::Output<stm32h7xx_hal::gpio::PushPull>,
//             >,
//         >,
//     >,
// > = cortex_m::interrupt::Mutex::new(core::cell::RefCell::new(None));
// pub fn led_init() {
//     cortex_m::interrupt::free(|cs| {
//         let _dp = DP.borrow(cs).take().unwrap();
//         let _ccdr = CCDR.borrow(cs).take().unwrap();
//         let _gpioe = _dp.GPIOE.split(_ccdr.peripheral.GPIOE);
//         let mut _led = _gpioe.pe3.into_push_pull_output();
//         // _led.make_interrupt_source(&mut syscfg);
//         // _led.enable_interrupt(&mut dp.EXTI);
//         // _led.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
//         led.borrow(cs).replace(Some(_led));
//     });
//     cortex_m::interrupt::free(|cs| {
//         let mut _cp = CP.borrow(cs).take().unwrap();
//         unsafe {
//             _cp.NVIC.set_priority(stm32h7xx_hal::interrupt::EXTI3, 1);
//             cortex_m::peripheral::NVIC::unmask::<stm32h7xx_hal::interrupt>(
//                 stm32h7xx_hal::interrupt::EXTI3,
//             );

//             _cp.NVIC.set_priority(stm32h7xx_hal::interrupt::EXTI9_5, 1);
//             cortex_m::peripheral::NVIC::unmask::<stm32h7xx_hal::interrupt>(
//                 stm32h7xx_hal::interrupt::EXTI9_5,
//             );
//         }
//     })
// }
