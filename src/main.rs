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
use utilities::{gpio, init, serial, time, wrap};
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

pub static WIFI_TX: cortex_m::interrupt::Mutex<
    core::cell::RefCell<
        core::option::Option<
            stm32h7xx_hal::gpio::gpioa::PA9<
                stm32h7xx_hal::gpio::Alternate<stm32h7xx_hal::gpio::AF7>,
            >,
        >,
    >,
> = cortex_m::interrupt::Mutex::new(core::cell::RefCell::new(None));
#[wrap(home(led))]
fn test() {
    cortex_m::interrupt::free(|cs| {
        let led = home::WIFI_TX.borrow(cs).borrow_mut().as_mut();
    });
}

// #[init(home(led, "wifi_rx",wifi_tx))]
#[entry]
fn main() -> ! {
    loop {}
}
