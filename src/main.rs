// Following tutorial `https://os.phil-opp.com`
// Currently at `https://os.phil-opp.com/paging-implementation/#accessing-the-page-tables`

#![no_std]
#![no_main]
#![feature(decl_macro, abi_x86_interrupt, let_chains, panic_info_message)]
#![allow(unused_parens)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "init_test"]
mod test;


use core::panic::{PanicInfo, Location};

use x86_64::instructions;
use bootloader::{
    BootInfo,
    entry_point
};

mod vga;
mod interrupt;
mod init;


// Freeze and do nothing on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga::colour!();
    vga::print!("\n");
    vga::colour!(White, Red);
    vga::print!("{:width$}\n",
        "KERNEL PANICKED",
        width=vga::BUFFER_SIZE.0
    );
    vga::print!("at {:width$}\n",
        if let Some(location) = info.location() {
            location
        } else {Location::caller()},
        width=(vga::BUFFER_SIZE.0 -3)
    );
    vga::print!("{:width$}",
        if let Some(message) = info.message() {
            message.as_str().unwrap()
        } else {""},
        width=vga::BUFFER_SIZE.0
    );
    vga::colour!();
    loop {
        instructions::hlt();
    }
}


// Entry
entry_point!(entry);
pub fn entry(info : &'static BootInfo) -> ! {
    interrupt::init();

    #[cfg(test)]
    init_test();

    init::main();

    loop {
        instructions::hlt();
    }
}
