#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
extern crate alloc;
use core::alloc::Layout;
use core::cell::UnsafeCell;
use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::entry;
use home::bump_alloc::BumpPointerAlloc;
use utilities::{gpio, time};
#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    asm::bkpt();
    loop {}
}
#[global_allocator]
static HEAP: BumpPointerAlloc = BumpPointerAlloc {
    head: UnsafeCell::new(0x2000_0100),
    end: 0x2000_0200,
};
#[panic_handler]
fn cortex_panic_handler(_panic: &PanicInfo) -> ! {
    loop {}
}
// time!(100);
gpio!(

    [
        {
            name:"led",gpio_group:"gpioe",pin:3,
            mode:"push_pull",interrupt:"EXIT3",priority:"handle_exit"
        },
        {
            name:"red",gpio_group:"gpioec",pin:5,
            mode:"push_pull",interrupt:"EXIT3",priority:"handle_exit_second"
        },
        {
            name:"ced",gpio_group:"gpioec",pin:5,
            mode:"push_pull",interrupt:"EXIT3",priority:"handle_exit_second"
        }
    ]

);

#[entry]
fn main() -> ! {
    loop {}
}
