use crate::key_value::KeyValue;
#[derive(Debug)]
struct SerialStruct {
    name: Option<syn::Ident>,
    usart: Option<syn::Ident>,
    tx: Option<syn::Ident>,
    rx: Option<syn::Ident>,
    baud_rate: Option<syn::Ident>,
}

fn convert_map_to_serial_struct(key_values: Vec<KeyValue>) -> Result<SerialStruct, ()> {
    let mut name: Option<syn::Ident> = None;
    let mut usart: Option<syn::Ident> = None;
    let mut tx: Option<syn::Ident> = None;
    let mut rx: Option<syn::Ident> = None;
    let mut baud_rate: Option<syn::Ident> = None;
    for key_value in key_values {
        // eprintln!("{},{}",key_value.key,key_value.value.to_string());
        match key_value.key.as_str() {
            "name" => name = Some(key_value.value),
            "usart" => usart = Some(key_value.value),
            "tx" => tx = Some(key_value.value),
            "rx" => rx = Some(key_value.value),
            "baud_rate" => baud_rate = Some(key_value.value),
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
    Ok(SerialStruct {
        name,
        usart,
        rx,
        tx,
        baud_rate,
    })
}

pub fn convert_serial_struct_to_quote(
    key_values: Vec<KeyValue>,
) -> Result<proc_macro2::TokenStream, String> {
    let serial_struct = match convert_map_to_serial_struct(key_values) {
        Ok(ok) => ok,
        Err(_) => return Err("".to_string()),
    };
    // 处理静态变量名字和函数名字
    let name = match serial_struct.name {
        Some(name) => syn::Ident::new(name.to_string().to_uppercase().as_str(), name.span()),
        None => return Err(String::from("must have name")),
    };
    let _function_name_literal = format!("{}_init", name.to_string().to_lowercase());
    let _function_name_ident = syn::Ident::new(_function_name_literal.as_str(), name.span());
    //处理tx
    let tx = match serial_struct.tx {
        Some(tx) => {
            let tx_uppercase=tx.to_string().to_uppercase();
            syn::Ident::new(tx_uppercase.as_str(), tx.span())
        },
        None => return Err(String::from("must have name")),
    };
    //处理rx
    let rx = match serial_struct.rx {
        Some(rx) => {
            let rx_uppercase=rx.to_string().to_uppercase();
            syn::Ident::new(rx_uppercase.as_str(), tx.span())
        },
        None => return Err(String::from("must have name")),
    };
    //处理波特率
    let  _baud_rate:u32;
    let mut _baud_rate_string = match serial_struct.baud_rate {
        Some(baud_rate) => baud_rate.to_string(),
        None => return Err(String::from("must have name")),
    };
    if let Some(idx) = _baud_rate_string.find('_'){
        _baud_rate=_baud_rate_string.split_off(idx+1).parse().unwrap();
    }else{
        return Err(String::from("must have name"))
    }
    //处理usart
    let usart = match serial_struct.usart {
        Some(usart) => usart,
        None => return Err(String::from("must have name")),
    };
    let mut ret = proc_macro2::TokenStream::new();
    let static_variable = quote::quote! {
        pub static #name: cortex_m::interrupt::Mutex<
            core::cell::RefCell<
                core::option::Option<
                    stm32h7xx_hal::serial::Serial<
                        stm32h7xx_hal::device::#usart
                        >
                    >,
                >,
            > = cortex_m::interrupt::Mutex::new(core::cell::RefCell::new(None));
    };

    let initialization_function = quote::quote! {
        pub fn #_function_name_ident() {
                cortex_m::interrupt::free(|cs| {
                    let mut dp = DP.borrow(cs).take().unwrap();
                    let mut ccdr = CCDR.borrow(cs).take().unwrap();
                    let tx = #tx.borrow(cs).take().unwrap();
                    let rx = #rx.borrow(cs).take().unwrap();
                    let serial = dp
                        .#usart
                        .serial((tx, rx), #_baud_rate.bps(), ccdr.peripheral.#usart, &ccdr.clocks)
                        .unwrap();
                        #name.borrow(cs).replace(Some(serial))
                });
            }
    };
    ret.extend(static_variable);
    ret.extend(initialization_function);
    return Ok(ret);
}
