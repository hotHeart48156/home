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
// use utilities::{gpio, init, serial, time, wrap};
use utilities::wrap;
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

#[entry]
fn main() -> ! {
    loop {}
}
