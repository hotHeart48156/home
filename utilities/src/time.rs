pub struct TimeParse {
    pub time: u32,
}
impl TimeParse {
    pub fn expand(&self) -> proc_macro2::TokenStream {
        let mut ret = proc_macro2::TokenStream::new();
        let static_variable = quote::quote! {
        use stm32h7xx_hal::prelude::*;
        pub static CP: cortex_m::interrupt::Mutex<core::cell::RefCell<core::option::Option<cortex_m::Peripherals>>> = cortex_m::interrupt::Mutex::new(core::cell::RefCell::new(None));
        pub static DP: cortex_m::interrupt::Mutex<core::cell::RefCell<core::option::Option<stm32h7xx_hal::pac::Peripherals>>> = cortex_m::interrupt::Mutex::new(core::cell::RefCell::new(None));
        pub static PWR: cortex_m::interrupt::Mutex<core::cell::RefCell<core::option::Option<stm32h7xx_hal::pwr::Pwr>>> = cortex_m::interrupt::Mutex::new(core::cell::RefCell::new(None));
        pub static PWR_CONF: cortex_m::interrupt::Mutex<core::cell::RefCell<core::option::Option<stm32h7xx_hal::pwr::PowerConfiguration>>> = cortex_m::interrupt::Mutex::new(core::cell::RefCell::new(None));
        pub static RCC: cortex_m::interrupt::Mutex<core::cell::RefCell<core::option::Option<stm32h7xx_hal::rcc::Rcc>>> = cortex_m::interrupt::Mutex::new(core::cell::RefCell::new(None));
        pub static CCDR: cortex_m::interrupt::Mutex<core::cell::RefCell<core::option::Option<stm32h7xx_hal::rcc::Ccdr>>> = cortex_m::interrupt::Mutex::new(core::cell::RefCell::new(None));
        };
        let frequence = self.time;
        let initialization_function = quote::quote! {
            pub fn time_init(){
                let mut per = cortex_m::Peripherals::take();
                let pp = stm32h7xx_hal::pac::Peripherals::take();
                cortex_m::interrupt::free(|cs| {
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
                    let _ccdr = _ccdr_rcc.sys_ck(#frequence.mhz()).freeze(_pwr_cfg, &_dp.SYSCFG);
                    CCDR.borrow(cs).replace(Some(_ccdr));
                    //中断
                    // let _dp=DP.borrow(cs).take().unwrap();
                    // let mut exti = _dp.EXTI;
                });
                // Ok(())
            }
        };
        ret.extend(static_variable);
        ret.extend(initialization_function);
        return ret;
    }
}
impl syn::parse::Parse for TimeParse {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let frequence: syn::LitInt = input.parse()?;
        return Ok(TimeParse {
            time: frequence.base10_parse()?,
        });
    }
}
