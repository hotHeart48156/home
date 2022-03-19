#![no_std]
#![feature(alloc)]
#![feature(alloc_error_handler)]

pub mod  bump_alloc;
extern crate alloc;
extern crate cortex_m;
use alloc::{vec::Vec, string::String};
use core::alloc::GlobalAlloc;
use core::ptr;
use core::cell::UnsafeCell;
use cortex_m::interrupt;
use cortex_m::asm;
use core::alloc::Layout;


// Declaration of the global memory allocator
// NOTE the user must ensure that the memory region `[0x2000_0100, 0x2000_0200]`
// is not used by other parts of the program

enum GpioMode {
    PushPull
} 
struct GpioConf{
    name:String,
    gpio:String,
    mode:GpioMode
}
struct DeviceConf {
    system_clock: u32,

    gpios: Option<Vec<GpioConf>>,
}
