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
use utilities::{gpio, serial, time};
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
time!(72);
gpio!(
    [
        {
            name:led,gpio_group:"gpioe",pin:3,
            mode:"push_pull_output",interrupt:"EXTI3",priority:1
        },
        {
            name:"wifi_tx",gpio_group:"gpioa",pin:9,
            mode:"alternate_af7",interrupt:"EXTI3",priority:1
        },
        {
            name:"wifi_rx",gpio_group:"gpioa",pin:10,
            mode:"alternate_af7",interrupt:"EXTI4",priority:1
        },
        {
            name:"usb",gpio_group:"gpioe",pin:3,
            mode:"alternate_af7",interrupt:"EXTI3",priority:1
        }
    ]
);
serial!(
    [
        {
            name:"wifi",usart:"USART1",tx:"wifi_tx",rx:"wifi_rx",baud_rate:115200
        }
    ]
);

#[entry]
fn main() -> ! {
    loop {}
}
