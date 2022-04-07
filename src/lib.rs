#![no_std]
pub mod bump_alloc;
pub mod mqtt;
pub mod wifi;
extern crate alloc;
extern crate cortex_m;
use utilities::{gpio, serial, time};
time!(100);
gpio!(
    [
        {
            name:"led",gpio_group:"gpioe",pin:1,
            mode:"push_pull_output"
        },
        {
            name:"wifi_tx",gpio_group:"gpioa",pin:2,
            mode:"alternate_af7",interrupt:"EXTI2",priority:1
        },
        {
            name:"wifi_rx",gpio_group:"gpioa",pin:3,
            mode:"alternate_af7",interrupt:"EXTI2",priority:1
        },
        {
            name:"usb_tx",gpio_group:"gpioa",pin:9,
            mode:"alternate_af7",interrupt:"EXTI3",priority:1
        },
        {
            name:"usb_rx",gpio_group:"gpioa",pin:10,
            mode:"alternate_af7",interrupt:"EXTI4",priority:1
        }
    ]
);
serial!(
    [
        {
            name:"wifi",usart:"USART2",tx:"wifi_tx",rx:"wifi_rx",baud_rate:115200
        },
        {
            name:"usb",usart:"USART1",tx:"usb_tx",rx:"usb_rx",baud_rate:115200
        }
    ]
);
