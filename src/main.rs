#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        on_update();
    }
}

fn on_update() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic!("{}", info);
}
