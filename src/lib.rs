#![no_std]
pub mod bump_alloc;
pub mod utiliti;
// pub mod interrupt;
extern crate alloc;
extern crate cortex_m;
use alloc::{string::String, vec::Vec};
use core::borrow::Borrow;
use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use cortex_m::peripheral::NVIC;
use stm32h7xx_hal::gpio::Edge;
use stm32h7xx_hal::gpio::{gpioe::PE3, Input, PullUp};
use stm32h7xx_hal::pwr::{PowerConfiguration, Pwr};
use stm32h7xx_hal::rcc::{Ccdr, Rcc};
use stm32h7xx_hal::rng::Rng;
use stm32h7xx_hal::{interrupt, pac, prelude::*};
use utilities::gpio;
gpio!(

    [
        {
            name:"led",gpio_group:"gpioe",pin:3,
            mode:"pushpull",interrput:"EXIT3",inter_fun:"handle_exit"
        },
        {
            name:"red",gpio_group:"gpioec",pin:5,
            mode:"pushpull",interrput:"EXIT3",inter_fun:"handle_exit_second"
        },
        {
            name:"red",gpio_group:"gpioec",pin:5,
            mode:"pushpull",interrput:"EXIT3",inter_fun:"handle_exit_second"
        }
    ]

);
static led: Mutex<RefCell<Option<PE3<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
static CP: Mutex<RefCell<Option<cortex_m::Peripherals>>> = Mutex::new(RefCell::new(None));
static DP: Mutex<RefCell<Option<pac::Peripherals>>> = Mutex::new(RefCell::new(None));
static PWR: Mutex<RefCell<Option<Pwr>>> = Mutex::new(RefCell::new(None));
static PWR_CONF: Mutex<RefCell<Option<PowerConfiguration>>> = Mutex::new(RefCell::new(None));
static RCC: Mutex<RefCell<Option<Rcc>>> = Mutex::new(RefCell::new(None));
static CCDR: Mutex<RefCell<Option<Ccdr>>> = Mutex::new(RefCell::new(None));
fn time_init() -> Result<(), ()> {
    let mut per = cortex_m::Peripherals::take();
    let pp = pac::Peripherals::take();
    free(|cs| {
        //总线
        DP.borrow(cs).replace(pp);
        CP.borrow(cs).replace(per);
        let per = CP.borrow(cs).take().unwrap();
        //电源
        let pwr_rng = DP.borrow(cs).take().unwrap().PWR.constrain();
        PWR.borrow(cs).replace(Some((pwr_rng)));
        let _pwr_rng = DP.borrow(cs).take().unwrap().PWR.constrain();
        PWR_CONF.borrow(cs).replace(Some(_pwr_rng.freeze()));
        //时钟rcc
        let _rcc = DP.borrow(cs).take().unwrap().RCC.constrain();
        RCC.borrow(cs).replace(Some(_rcc));
        //时钟ccdr
        let _pwr_cfg = PWR_CONF.borrow(cs).take().unwrap();
        let _ccdr_rcc = RCC.borrow(cs).take().unwrap();
        let _dp = DP.borrow(cs).take().unwrap();
        let _ccdr = _ccdr_rcc.sys_ck(100.mhz()).freeze(_pwr_cfg, &_dp.SYSCFG);
        CCDR.borrow(cs).replace(Some(_ccdr));
        //中断
        // let _dp=DP.borrow(cs).take().unwrap();
        // let mut exti = _dp.EXTI;
    });
    Ok(())
}
fn led_init() {
    free(|cs| {
        let _dp = DP.borrow(cs).take().unwrap();
        let _ccdr = CCDR.borrow(cs).take().unwrap();
        let _gpioe = _dp.GPIOE.split(_ccdr.peripheral.GPIOE);
        let mut _led = _gpioe.pe3.into_pull_up_input();
        // _led.make_interrupt_source(&mut syscfg);
        // _led.enable_interrupt(&mut dp.EXTI);
        // _led.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
        led.borrow(cs).replace(Some(_led));
    });
    free(|cs| {
        let mut _cp = CP.borrow(cs).take().unwrap();
        unsafe {
            _cp.NVIC.set_priority(interrupt::EXTI3, 1);
            NVIC::unmask::<interrupt>(interrupt::EXTI3);

            _cp.NVIC.set_priority(interrupt::EXTI9_5, 1);
            NVIC::unmask::<interrupt>(interrupt::EXTI9_5);
        }
    })
}
