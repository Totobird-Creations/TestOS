#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(test_os::test::runner)]
#![reexport_test_harness_main = "test_main"]

extern crate test_os;
extern crate alloc;

use alloc::{
    boxed::Box,
    vec::Vec
};

use test_os::mem::allocator::HEAP_SIZE;


// Simple allocation.
#[test_case]
fn test_simple_alloc() {
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
