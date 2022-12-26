#![no_std] 
#![no_main] 
mod vga_buf;


use core::fmt::Write;
use core::panic::PanicInfo;

use crate::vga_buf::{Alignment, Screen};
use crate::vga_buf::Color::{Black, LightGreen, White};


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] 
pub extern "C" fn _start() -> ! {
    let mut screen = Screen::new(LightGreen, Black, Alignment::Center);

    for i in 0..100 {
        let _ = write!(screen, "Number {}\n", i);
    }

    

    loop {}
}
