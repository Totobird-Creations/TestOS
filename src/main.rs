// Following tutorial `https://os.phil-opp.com`
//
// Build with `cargo bootimage`

#![no_std]
#![no_main]
#![feature(decl_macro, abi_x86_interrupt)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "init_test"]
mod test;


use core::panic::PanicInfo;

mod vga;
mod exception;
mod init;


// Freeze and do nothing on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry
#[no_mangle]
pub extern "C" fn _start() -> ! {
    exception::init();

    #[cfg(test)]
    init_test();

    init::main();

    loop {}
}
