// Following tutorial `https://os.phil-opp.com`
// Currently at `https://os.phil-opp.com/heap-allocation/#adding-a-test`

#![no_std]
#![no_main]
#![feature(
    decl_macro,
    abi_x86_interrupt,
    let_chains,
    panic_info_message,
    alloc_error_handler
)]
#![allow(unused_parens)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "init_test"]
pub mod test;


extern crate core;
extern crate volatile;
extern crate spin;
extern crate x86_64;
extern crate pic8259;
extern crate pc_keyboard;
extern crate lazy_static;
extern crate bootloader;
extern crate alloc;
extern crate linked_list_allocator;
extern crate uart_16550;

use core::panic::{PanicInfo, Location};

use x86_64::instructions;
use bootloader::{
    BootInfo,
    entry_point
};

mod vga;
mod interrupt;
mod init;
mod info;
pub mod mem;


// Freeze and do nothing on panic.
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
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
            if let Some(message) = message.as_str() {
                message
            } else {""}
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
fn entry(info : &'static BootInfo) -> ! {
    init(info);

    #[cfg(test)]
    init_test();

    init::main();

    loop {
        instructions::hlt();
    }
}

pub fn init(info : &'static BootInfo) {
    info::load(info);

    mem::init().unwrap();
    interrupt::init();
}
