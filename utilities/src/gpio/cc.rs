use std::collections::HashMap;

use crate::key_value::KeyValue;
struct GpioStruct {
    name: Option<String>,
    mode: Option<String>,
    gpio_group: Option<String>,
    pin: Option<String>,
    interrupt: Option<String>,
    priority: Option<String>,
}

struct Mode<'a> {
    forward: &'a str,
    mode: &'a str,
    function: &'a str,
}

fn parse(key_values: Vec<KeyValue>) -> GpioStruct {
    let MODE_MAP: HashMap<&str, Mode> = HashMap::from([
        (
            "push_pull",
            Mode {
                forward: "Input",
                mode: "Pull",
                function: "into_pull_up_input",
            },
        ),
        (
            "open_drain",
            Mode {
                forward: "Input",
                mode: "Pull",
                function: "into_open_drain",
            },
        ),
    ]);
    let mut name: Option<String> = None;
    let mut gpio_group: Option<String> = None;
    let mut mode: Option<String> = None;
    let mut pin: Option<String> = None;
    let mut interrupt: Option<String> = None;
    let mut priority: Option<String> = None;

    for key_value in key_values {
        match key_value.key.as_str() {
            "name" => name = Some(key_value.value),
            "gpio_group" => gpio_group = Some(key_value.value),
            "pin" => pin = Some(key_value.value),
            "interrupt" => interrupt = Some(key_value.value),
            "priority" => priority = Some(key_value.value),
            "mode" => mode = Some(key_value.value),
            &_ => name = Some("".to_string()),
        }
    }
    GpioStruct {
        name,
        gpio_group,
        pin,
        interrupt,
        priority,
        mode,
    }
}

fn parsecc() -> Result<proc_macro2::TokenStream, String> {
    let key_values: Vec<KeyValue> = vec![];
    let gpio_struct = parse(key_values);
    let name = match gpio_struct.name {
        Some(name) => name,
        None => return Err(String::from("must have name")),
    };
    let mode = match gpio_struct.mode {
        Some(mode) => mode,
        None => {
            return Err("".to_string());
        }
    };
    let gpio_group = match gpio_struct.gpio_group {
        Some(gpio_group) => gpio_group,
        None => {
            return Err("".to_string());
        }
    };
    let pin = match gpio_struct.pin {
        Some(pin) => pin,
        None => {
            return Err("".to_string());
        }
    };
    let interrupt = match gpio_struct.interrupt {
        Some(interrupt) => interrupt,
        None => String::from(""),
    };
    let priority = match gpio_struct.priority {
        Some(priority) => priority,
        None => String::from(""),
    };

    let ident_mode = MODE_MAP.get(mode);
    let forward = ident_mode.forward; //此处不能直接在quote中用点。
    let mode = ident_mode.mode;
    let mut static_quote_block = quote::quote! {
    static #name: Mutex<RefCell<Option<PE3<#forward<#mode>>>>> = Mutex::new(RefCell::new(None));
    };

    let init_quote_function = quote::quote! {
        fn #name_init() {
            free(|cs| {
                let _dp = DP.borrow(cs).take().unwrap();
                let _ccdr = CCDR.borrow(cs).take().unwrap();
                let _gpioe = _dp.GPIOE.split(_ccdr.peripheral.GPIOE);
                let mut _led = _gpioe.pe3.into_pull_up_input();
                // _led.make_interrupt_source(&mut syscfg);
                // _led.enable_interrupt(&mut dp.EXTI);
                // _led.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
                name.borrow(cs).replace(Some(_led));
            });
            free(|cs| {
                let mut _cp = CP.borrow(cs).take().unwrap();
                unsafe {
                    _cp.NVIC.set_priority(interrupt::#interrupt, #priority);
                    NVIC::unmask::<interrupt>(interrupt::#interrupt);
                    _cp.NVIC.set_priority(interrupt::#interrupt, #priority);
                    NVIC::unmask::<interrupt>(interrupt::#interrupt,);
                }
            });
        }

    };
    static_quote_block.extend(init_quote_function);
    return Ok(static_quote_block);
}
