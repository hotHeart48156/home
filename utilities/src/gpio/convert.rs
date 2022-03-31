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
                forward: "Output",
                mode: "PushPull",
                function: "into_push_pull_output",
            },
        ),
        (
           String::from("open_drain"),
            Mode {
                forward: "Output",
                mode: "PushPull",
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
    // 处理静态变量名字和函数名字
    let name = match gpio_struct.name {
        Some(name) => syn::Ident::new(name.to_string().to_uppercase().as_str(), name.span()),
        None => return Err(String::from("must have name")),
    };
    let _function_name_literal = format!("{}_init", name.to_string().to_lowercase());
    let _function_name_ident = syn::Ident::new(_function_name_literal.as_str(), name.span());
    //处理gpio模式
    let _mode = match gpio_struct.mode {
        Some(mode) => mode,
        None => {
            return Err("".to_string());
        }
    };
    let mode_struct = match MODE_MAP.get(&String::from("push_pull")) {
        Some(mode) => mode,
        None => {
            eprintln!("not support mode");
            return Err(format!("mode not found"));
        }
    };
    let _mode_literal = mode_struct.mode;
    let _mode_ident = syn::Ident::new(_mode_literal, _mode.span());

    let _forward_literal = mode_struct.forward;
    let _forward_ident = syn::Ident::new(_forward_literal, _mode.span());

    let _mode_function_literal = mode_struct.function;
    let _mode_function_ident = syn::Ident::new(_mode_function_literal, _mode.span());
    //处理gpio组,和引脚
    let _gpio_group = match gpio_struct.gpio_group {
        Some(gpio_group) => gpio_group,
        None => {
            return Err("".to_string());
        }
    };
    let _pin_string = match gpio_struct.pin {
        Some(pin) => pin.to_string(),
        None => {
            return Err("".to_string());
        }
    };
    let _pin_number = _pin_string.chars().last().unwrap();
    let _gpio_group_last_char = _gpio_group
        .clone()
        .to_string()
        .chars()
        .last()
        .unwrap()
        .to_string();
    let _gpio_group_last_char_uppercase = _gpio_group_last_char.to_uppercase();
    let _gpio_group_last_char_lowercase = _gpio_group_last_char.to_lowercase();

    let _gpio_group_literal_lowercase =
        format!("{}{}", "gpio", _gpio_group_last_char_lowercase).to_lowercase();
    let _gpio_group_literal_uppercase =
        format!("{}{}", "GPIO", _gpio_group_last_char_uppercase).to_uppercase();
    let _gpio_group_ident_lowercase =
        syn::Ident::new(_gpio_group_literal_lowercase.as_str(), _gpio_group.span());
    let _gpio_group_ident_uppercase =
        syn::Ident::new(_gpio_group_literal_uppercase.as_str(), _gpio_group.span());
    let _gpio_group_pin_literal_uppercase = format!(
        "{}{}{}",
        "P",
        _gpio_group_last_char_uppercase.to_owned(),
        _pin_number
    )
    .to_uppercase();
    let _gpio_group_pin_ident_uppercase = syn::Ident::new(
        _gpio_group_pin_literal_uppercase.as_str(),
        _gpio_group.span(),
    );

    let _gpio_group_pin_literal_lowercase = format!(
        "{}{}{}",
        "p",
        _gpio_group_last_char_uppercase.to_owned(),
        _pin_number
    )
    .to_lowercase();
    let _gpio_group_pin_ident_lowercase = syn::Ident::new(
        _gpio_group_pin_literal_lowercase.as_str(),
        _gpio_group.span(),
    );

    //处理gpio中断
    let _interrupt = match gpio_struct.interrupt {
        Some(interrupt) => interrupt,
        None => return Err("".to_string()),
    };
    //处理gpio中断优先级
    let _priority = match gpio_struct.priority {
        Some(priority) => priority,
        None => return Err("".to_string()),
    };

    let mut ret = proc_macro2::TokenStream::new();
    let static_variable = quote::quote! {
        pub static #name: cortex_m::interrupt::Mutex<
            core::cell::RefCell<
                core::option::Option<
                    stm32h7xx_hal::gpio::#_gpio_group_ident_lowercase::#_gpio_group_pin_ident_uppercase<
                        stm32h7xx_hal::gpio::#_forward_ident<
                            stm32h7xx_hal::gpio::#_mode_ident
                        >,
                    >,
                >,
            >,
        > = cortex_m::interrupt::Mutex::new(core::cell::RefCell::new(None));
    };

    let initialization_function = quote::quote! {
        pub fn #_function_name_ident() {
            cortex_m::interrupt::free(|cs| {
                let _dp = DP.borrow(cs).take().unwrap();
                let _ccdr = CCDR.borrow(cs).take().unwrap();
                let _gpio = _dp.#_gpio_group_ident_uppercase.split(_ccdr.peripheral.#_gpio_group_ident_uppercase);
                let mut _led = _gpio.#_gpio_group_pin_ident_lowercase.#_mode_function_ident();
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
                }
            })
        }
    };
    ret.extend(static_variable);
    ret.extend(initialization_function);
    return Ok(ret);
}
