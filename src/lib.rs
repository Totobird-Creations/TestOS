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


extern crate alloc;


use core::panic::{PanicInfo, Location};

use x86_64::instructions;
use bootloader::BootInfo;

pub mod vga;
mod     interrupt;
pub mod init;
mod     info;
pub mod mem;
mod     tasks;


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


pub fn init(info : &'static BootInfo) {
    info::load(info);

    mem::init().unwrap();
    interrupt::init();
}
