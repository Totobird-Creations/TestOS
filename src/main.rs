// Following tutorial `https://os.phil-opp.com`
//
// Build with `cargo bootimage`

#![no_std]
#![no_main]
#![feature(decl_macro)]

use core::panic::PanicInfo;

mod vga;


// Freeze and do nothing on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry
#[no_mangle]
pub extern "C" fn _start() -> ! {

    vga::println("Text printed to screen.");
    vga::println("Next line.");
    vga::println("Hello World!");

    loop {}
}