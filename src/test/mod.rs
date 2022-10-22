use core::any::type_name;

use crate::vga;

pub mod qemu;
pub mod serial;


pub fn runner(tests : &[&dyn Testable]) {
    vga::colour!(LightCyan, Black);
    vga::print!("Running {} tests.", tests.len());
    vga::colour!();
    vga::print!("\n");
    for i in 0..tests.len() {
        tests[i].run();
    }
    vga::colour!(LightCyan, Black);
    vga::print!("Done.");
    vga::colour!();
    vga::print!("\n");
}

pub trait Testable {
    fn run(&self);
}
impl <T> Testable for T
    where T : Fn(),
{
    fn run(&self) {
        vga::colour!(Cyan, Black);
        vga::print!("{}", type_name::<T>());
        vga::colour!(DarkGray, Black);
        vga::print!(" ... ");
        vga::colour!();
        self();
        vga::colour!(LightGreen, Black);
        vga::print!("[OK]");
        vga::colour!();
        vga::print!("\n");
    }
}

#[test_case]
fn test_trivial() {
    assert_eq!(1, 1);
}

#[test_case]
fn test_vga_buffer() {
    vga::print!("simple_output ");
}

#[test_case]
fn test_exceptions() {
    // CPU Interrupt
    //x86_64::instructions::interrupts::int3();
    // Double Fault
    //unsafe {
    //    *(0xbadbadbad as *mut u64) = 42;
    //};
    // TODO : Stack overflow test
}
