use alloc::vec::{Vec};
use alloc::vec;
use utilities::wrap;
use core::fmt::Write;
use stm32h7xx_hal::{interrupt, hal::digital::v2::OutputPin};
#[wrap(crate(wifi,led))]
pub fn test(){
    if let Some(w) = wifi {
            let (mut tx,_)=w.split();
            write!(tx,"hello").unwrap();
    }
    if let Some(mut l)=led{
    }

}
#[interrupt]
fn EXTI2(){}