#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// default start function (for macOS)
#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}
