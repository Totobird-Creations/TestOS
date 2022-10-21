#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(test_os::test::runner)]
#![reexport_test_harness_main = "init_test"]

extern crate test_os;
extern crate alloc;

use x86_64::instructions;
use alloc::{
    boxed::Box,
    vec::Vec
};
use bootloader::{
    entry_point,
    BootInfo
};

use test_os::mem::allocator::HEAP_SIZE;


use test_os::init;

entry_point!(entry);
fn entry(info : &'static BootInfo) -> ! {
    init(info);

    init_test();

    loop {
        instructions::hlt();
    }
}


// Simple allocation.
#[test_case]
fn test_simple_alloc() {
    test_os::vga::print!("run");
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

// If memory wasn't properly being freed, this will crash
// because it will run out of memory.
#[test_case]
fn test_dealloc() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

// Tests large allocations.
#[test_case]
fn test_large_alloc() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i); 
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}
