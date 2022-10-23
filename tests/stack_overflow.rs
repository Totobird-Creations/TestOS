#![no_std]
#![no_main]
#![cfg(test)]

#![feature(custom_test_frameworks)]
#![test_runner(test_os::test::runner)]
#![reexport_test_harness_main = "init_test"]

extern crate test_os;

use bootloader::{
    entry_point,
    BootInfo
};

use test_os::{
    init_expect_panic,
    vga,
    test::qemu
};


entry_point!(entry);
fn entry(info : &'static BootInfo) -> ! {
    init_expect_panic(info);
    init_test();

    qemu::exit(qemu::QemuExitCode::Failure);
}


#[test_case]
fn test_stack_overflow() {
    vga::warn!("A DOUBLE FAULT ERROR IS EXPECTED HERE:");
    #[allow(unconditional_recursion)]
    fn stack_overflow() -> ! {stack_overflow();}
    stack_overflow();
}
