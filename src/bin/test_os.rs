#![no_std]
#![no_main]

use x86_64::instructions;
use bootloader::{
    BootInfo,
    entry_point
};

use test_os::init;


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
