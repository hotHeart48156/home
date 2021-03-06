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
    static ref MODE_MAP: HashMap<&'static str,Mode<'static>> = HashMap::from([
        (
            "push_pull_output",
            Mode {
                forward: "Output",
                mode: "PushPull",
                function: "into_push_pull_output",
            },
        ),
        (
           "open_drain",
            Mode {
                forward: "Output",
                mode: "PushPull",
                function: "into_open_drain",
            },
        ),
        (
            "alternate_af7",
             Mode {
                 forward: "Alternate",
                 mode: "AF7",
                 function: "into_alternate_af7",
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
    // ???????????????????????????????????????
    let name = match gpio_struct.name {
        Some(name) => syn::Ident::new(name.to_string().to_uppercase().as_str(), name.span()),
        None => return Err(String::from("must have name")),
    };
    let _function_name_literal = format!("{}_init", name.to_string().to_lowercase());
    let _function_name_ident = syn::Ident::new(_function_name_literal.as_str(), name.span());
    //??????gpio??????
    let _mode = match gpio_struct.mode {
        Some(mode) => mode,
        None => {
            return Err("".to_string());
        }
    };
    let _mode_literal=_mode.to_string();
    let mode_struct = match MODE_MAP.get(&_mode_literal.as_str()) {
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
    //??????gpio???,?????????
    let _gpio_group = match gpio_struct.gpio_group {
        Some(gpio_group) => gpio_group,
        None => {
            return Err("".to_string());
        }
    };
    let mut _pin_string = match gpio_struct.pin {
        Some(pin) => pin.to_string(),
        None => {
            return Err("".to_string());
        }
    };
    let _pin_number:String;
    if let Some(idx) = _pin_string.find('_'){
        _pin_number=_pin_string.split_off(idx+1)
    }else{
        _pin_number="".to_string();
    }
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

    //??????gpio??????
    let interrupt_quote: proc_macro2::TokenStream;
    if let Some(_interrupt_ident) = gpio_struct.interrupt {
        //??????gpio???????????????
        let _priority = match gpio_struct.priority {
            Some(priority) => priority.to_string(),
            None => "0".to_string(),
        };
        let _priority_str = &_priority.as_str()[_priority.len() - 1..];
        let _priority_number = _priority_str.parse::<u8>().unwrap();
        interrupt_quote = quote::quote! {
            cortex_m::interrupt::free(|cs| {
                let mut _cp = CP.borrow(cs).take().unwrap();
                unsafe {
                    _cp.NVIC.set_priority(stm32h7xx_hal::interrupt::#_interrupt_ident, #_priority_number);
                    cortex_m::peripheral::NVIC::unmask::<stm32h7xx_hal::interrupt>(
                        stm32h7xx_hal::interrupt::#_interrupt_ident,
                    );
                }
            })
        };
    } else {
        interrupt_quote = quote::quote! {};
    }

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
                #interrupt_quote
            });

        }
    };
    ret.extend(static_variable);
    ret.extend(initialization_function);
    return Ok(ret);
}
