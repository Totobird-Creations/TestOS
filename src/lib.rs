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

use bootloader::BootInfo;

pub mod vga;
mod     interrupt;
pub mod init;
mod     info;
pub mod mem;
mod     tasks;


#[panic_handler]
pub fn panic(info : &PanicInfo) -> ! {
    if (info::expect_panic()) {

        vga::colour!(Green, Black);
        vga::println!("KERNEL CORRECTLY PANICKED");
        vga::println!("at {}",
            if let Some(location) = info.location() {
                location
            } else {Location::caller()}
        );
        vga::println!("{}",
            if let Some(message) = info.message() {
                if let Some(message) = message.as_str() {
                    message
                } else {""}
            } else {""}
        );
        vga::print!("\n");
        vga::colour!(LightGreen, Black);
        vga::print!("[OK]");
        vga::colour!();
        vga::colour!(LightCyan, Black);
        vga::println!("Done.");

    } else {

        vga::colour!();
        vga::println!();
        vga::colour!(White, Red);
        vga::println!("KERNEL PANICKED");
        vga::println!("at {}",
            if let Some(location) = info.location() {
                location
            } else {Location::caller()}
        );
        vga::println!("{}",
            if let Some(message) = info.message() {
                if let Some(message) = message.as_str() {
                    message
                } else {""}
            } else {""}
        );
        
    }
    vga::colour!();
    vga::print!("\n");
    test::qemu::exit(if (info::expect_panic()) {
        test::qemu::QemuExitCode::Success
    } else {
        test::qemu::QemuExitCode::Failure
    });
}


#[cfg(test)]
use bootloader;
#[cfg(test)]
bootloader::entry_point!(entry);
#[cfg(test)]
fn entry(info : &'static BootInfo) -> ! {
    init(info);
    init_test();

    test::qemu::exit(test::qemu::QemuExitCode::Success);
}


pub fn init(info : &'static BootInfo) {
    info::load(info);

    mem::init().unwrap();
    interrupt::init();

    //tasks::init();
}

pub fn init_expect_panic(info : &'static BootInfo) {
    unsafe {info::EXPECT_PANIC = true};
    init(info);
}
