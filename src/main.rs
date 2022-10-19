// Following tutorial `https://os.phil-opp.com`
//
// Build with `cargo bootimage`

#![no_std]
#![no_main]
#![feature(decl_macro, abi_x86_interrupt)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
mod test;


use core::panic::PanicInfo;

mod vga;
mod exception;


// Freeze and do nothing on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry
#[no_mangle]
pub extern "C" fn _start() -> ! {

    vga::print!("Text printed to screen.\n");
    vga::colour!(Red, Cyan);
    vga::print!("Hello ");
    vga::colour!(Blue, Green);
    vga::print!("World!");
    vga::colour!();
    vga::print!("\n");
    vga::colour!(Pink, Magenta);
    vga::print!("Third line.");
    vga::colour!();
    vga::print!("\n");

    loop {}
}
