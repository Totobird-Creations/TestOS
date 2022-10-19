use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame
};

use crate::vga;


pub fn setup(idt : &mut InterruptDescriptorTable) {
    idt.breakpoint.set_handler_fn(handler);
}


extern "x86-interrupt" fn handler(stack_frame : InterruptStackFrame) {
    vga::colour!(LightRed, Black);
    vga::print!("EXCEPTION : BREAKPOINT\n{:#?}\n", stack_frame);
    vga::colour!(White, Black);
}
