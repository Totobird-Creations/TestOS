// Following tutorial `https://os.phil-opp.com`

#![no_std]
#![no_main]
#![feature(decl_macro, abi_x86_interrupt)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "init_test"]
mod test;


use core::panic::PanicInfo;

use x86_64::instructions;

mod vga;
mod interrupt;
mod init;


// Freeze and do nothing on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        instructions::hlt();
    }
}

// Entry
#[no_mangle]
pub extern "C" fn _start() -> ! {
    interrupt::init();

    #[cfg(test)]
    init_test();

    init::main();

    loop {
        instructions::hlt();
    }
}
