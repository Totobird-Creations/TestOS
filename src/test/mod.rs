use core::any::type_name;

use crate::vga;
use crate::info::expect_panic;

pub mod qemu;
pub mod serial;


pub fn runner(tests : &[&dyn Testable]) {
    vga::colour!(LightCyan, Black);
    vga::println!("Running {} tests.", tests.len());
    vga::colour!();
    for i in 0..tests.len() {
        tests[i].run();
    }
    if (! expect_panic()) {
        vga::colour!(LightCyan, Black);
        vga::println!("Done.");
        vga::colour!();
    }
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
        vga::print!("\n");
        vga::print!("{}", type_name::<T>());
        vga::colour!(DarkGray, Black);
        vga::print!(" ... ");
        vga::colour!();
        self();
        if (! expect_panic()) {
            vga::colour!(LightGreen, Black);
            vga::print!("[OK]");
            vga::colour!();
        }
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
fn test_breakpoint() {
    use x86_64::instructions::interrupts;
    vga::warn!("A BREAKPOINT ERROR IS EXPECTED HERE:");
    interrupts::int3();
}
