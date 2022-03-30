use std::collections::HashMap;

use crate::key_value::KeyValue;
#[derive(Debug)]
struct GpioStruct {
    name: Option<syn::Ident>,
    mode: Option<syn::Ident>,
    gpio_group: Option<syn::Ident>,
    pin: Option<syn::Ident>,
    interrupt: Option<syn::Ident>,
    priority: Option<syn::Ident>,
}

struct Mode<'a> {
    forward: &'a str,
    mode: &'a str,
    function: &'a str,
}
lazy_static::lazy_static!(
    static ref MODE_MAP: HashMap<String,Mode<'static>> = HashMap::from([
        (
            String::from("push_pull"),
            Mode {
                forward: "Input",
                mode: "Pull",
                function: "into_pull_up_input",
            },
        ),
        (
           String::from("open_drain"),
            Mode {
                forward: "Input",
                mode: "Pull",
                function: "into_open_drain",
            },
        ),
    ]);
);
fn convert_map_to_gpio_struct(key_values: Vec<KeyValue>) -> Result<GpioStruct, ()> {
    let mut name: Option<syn::Ident> = None;
    let mut gpio_group: Option<syn::Ident> = None;
    let mut mode: Option<syn::Ident> = None;
    let mut pin: Option<syn::Ident> = None;
    let mut interrupt: Option<syn::Ident> = None;
    let mut priority: Option<syn::Ident> = None;
    for key_value in key_values {
        // eprintln!("{},{}",key_value.key,key_value.value.to_string());
        match key_value.key.as_str() {
            "name" => name = Some(key_value.value),
            "gpio_group" => gpio_group = Some(key_value.value),
            "pin" => pin = Some(key_value.value),
            "interrupt" => interrupt = Some(key_value.value),
            "mode" => mode = Some(key_value.value),
            "priority" => priority = Some(key_value.value),
            &_ => {
                eprintln!(
                    "nofound key{} no found value{}",
                    key_value.key,
                    key_value.value.to_string()
                );
                return Err(());
            }
        }
    }
    Ok(GpioStruct {
        name,
        gpio_group,
        pin,
        interrupt,
        priority,
        mode,
    })
}

pub fn convert_gpio_struct_to_quote(
    key_values: Vec<KeyValue>,
) -> Result<proc_macro2::TokenStream, String> {
    let gpio_struct = match convert_map_to_gpio_struct(key_values) {
        Ok(ok) => ok,
        Err(_) => return Err("".to_string()),
    };
    let name = match gpio_struct.name {
        Some(name) => syn::Ident::new(name.to_string().to_uppercase().as_str(), name.span()),
        None => return Err(String::from("must have name")),
    };
    let _mode = match gpio_struct.mode {
        Some(mode) => mode,
        None => {
            return Err("".to_string());
        }
    };
    let ident_mode = match MODE_MAP.get(&String::from("push_pull")) {
        Some(mode) => mode,
        None => {
            eprintln!("no found");
            return Err(format!("mode not found"));
        }
    };
    let _gpio_group = match gpio_struct.gpio_group {
        Some(gpio_group) => gpio_group,
        None => {
            return Err("".to_string());
        }
    };
    let _pin = match gpio_struct.pin {
        Some(pin) => pin,
        None => {
            return Err("".to_string());
        }
    };
    let _interrupt = match gpio_struct.interrupt {
        Some(interrupt) => interrupt,
        None => return Err("".to_string()),
    };
    let _priority = match gpio_struct.priority {
        Some(priority) => priority,
        None => return Err("".to_string()),
    };

    let mut ret = proc_macro2::TokenStream::new();
    let _forward = ident_mode.forward; //此处不能直接在quote中用点。
    let _mode = ident_mode.mode;
    let static_function = quote::quote! {
            pub static #name: cortex_m::interrupt::Mutex<
        core::cell::RefCell<
            core::option::Option<
                stm32h7xx_hal::gpio::gpioe::PE3<
                    stm32h7xx_hal::gpio::Output<
                        stm32h7xx_hal::gpio::PushPull
                    >,
                >,
            >,
        >,
    > = cortex_m::interrupt::Mutex::new(core::cell::RefCell::new(None));
        };
    let _function_name_literal = format!("{}_init", name.to_string().to_lowercase());
    let _function_name_ident = syn::Ident::new(_function_name_literal.as_str(), name.span());
    let initialization_function = quote::quote! {
        pub fn #_function_name_ident() {
            cortex_m::interrupt::free(|cs| {
                let _dp = DP.borrow(cs).take().unwrap();
                let _ccdr = CCDR.borrow(cs).take().unwrap();
                let _gpioe = _dp.GPIOE.split(_ccdr.peripheral.GPIOE);
                let mut _led = _gpioe.pe3.into_push_pull_output();
                // _led.make_interrupt_source(&mut syscfg);
                // _led.enable_interrupt(&mut dp.EXTI);
                // _led.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
                #name.borrow(cs).replace(Some(_led));
            });
            cortex_m::interrupt::free(|cs| {
                let mut _cp = CP.borrow(cs).take().unwrap();
                unsafe {
                    _cp.NVIC.set_priority(stm32h7xx_hal::interrupt::EXTI3, 1);
                    cortex_m::peripheral::NVIC::unmask::<stm32h7xx_hal::interrupt>(
                        stm32h7xx_hal::interrupt::EXTI3,
                    );

                    _cp.NVIC.set_priority(stm32h7xx_hal::interrupt::EXTI9_5, 1);
                    cortex_m::peripheral::NVIC::unmask::<stm32h7xx_hal::interrupt>(
                        stm32h7xx_hal::interrupt::EXTI9_5,
                    );
                }
            })
        }
    };
    ret.extend(static_function);
    ret.extend(initialization_function);
    return Ok(ret);
}
