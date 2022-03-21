#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
extern crate alloc;
use alloc::vec::Vec;
use cortex_m::interrupt::Mutex;
use core::alloc::Layout;
use core::cell::{UnsafeCell, Cell, RefCell};
use core::panic::PanicInfo;
use cortex_m::{asm, Peripherals};
use cortex_m_rt::entry;
use home::bump_alloc::BumpPointerAlloc;
use stm32h7xx_hal::hal::digital::v2::OutputPin;
use stm32h7xx_hal::{pac, prelude::*};
use utilities::config;
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
macro_rules! example_power {
    ($pwr:ident) => {{
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "smps", feature = "example-smps"))] {
                $pwr.smps()
            } else if #[cfg(all(feature = "smps", feature = "example-ldo"))] {
                $pwr.ldo()
            } else {
                $pwr
            }
        }
    }};
}
macro_rules! time {
    ($FREQ:expr) => {
        static cp=cortex_m::Peripherals::take().unwrap();
        static dp = pac::Peripherals::take().unwrap();
        static pwr = dp.PWR.constrain();
        static rcc = dp.RCC.constrain();
        
        static pwrcfg = pwr.freeze();
        // let ccdr = rcc.sys_ck($FREQ.mhz()).freeze(pwrcfg, &dp.SYSCFG);  
    };
}
macro_rules! gpio {
    () => {
    let gpioe = &dp.GPIOE.split(ccdr.peripheral.GPIOE);
    // let mut led= gpioe.pe1.into_push_pull_output();
    // let mut delay = cp.SYST.delay(ccdr.clocks);
    };
}
static COUNTER: Mutex<RefCell<Option<cortex_m::Peripherals>>> = Mutex::new(RefCell::new(Option::None));
#[entry]
fn main() -> ! {
    let cp=cortex_m::Peripherals::take().unwrap();
    // COUNTER.borrow(cp).borrow();
    
// static dp = pac::Peripherals::take().unwrap();
// static pwr = dp.PWR.constrain();
// static rcc = dp.RCC.constrain();
// static pwrcfg = pwr.freeze();
    // time!(100);
    // gpio!();
    
    // loop {
    //     loop {
    //         led.set_high().unwrap();
    //         delay.delay_ms(500_u16);

    //         led.set_low().unwrap();
    //         delay.delay_ms(500_u16);
    //     }
    // }
    #[config]
    const device:_ = DeviceConf {
        system_clock: 32,
        gpios: Some(Vec(
            GpioConf {
            name: "light",
            gpio_group: "gpioe",
            gpio_pin:3,
            mode: GpioMode::PushPull,
        },
        GpioConf {
            name: "light",
            gpio_group: "gpioe",
            gpio_pin:3,
            mode: GpioMode::PushPull,
        }
    )),
    };
    loop {}
}
