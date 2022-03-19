#![no_std]
pub mod  bump_alloc;
pub mod interrupt;
extern crate alloc;
extern crate cortex_m;
use alloc::{vec::Vec, string::String};
use core::alloc::GlobalAlloc;
use core::ptr;
use core::cell::UnsafeCell;
use cortex_m::interrupt;
use cortex_m::asm;
use core::alloc::Layout;
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
