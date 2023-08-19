#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        on_update();
    }
}

fn on_update(){}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

