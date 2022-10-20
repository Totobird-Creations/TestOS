use x86_64::{
    structures::idt::{
        InterruptDescriptorTable,
        InterruptStackFrame,
        PageFaultErrorCode
    },
    registers::control::Cr2
};

use crate::vga;


pub fn setup(idt : &mut InterruptDescriptorTable) {
    idt.page_fault.set_handler_fn(handler);
}


extern "x86-interrupt" fn handler(stack_frame : InterruptStackFrame, error_code : PageFaultErrorCode) {
    vga::colour!(LightRed, Black);
    vga::print!("EXCEPTION : PAGE FAULT - {:?}\n", error_code);
    vga::print!("            ADDRESS    - {:?}\n", Cr2::read());
    vga::print!("{:#?}\n", stack_frame);
    vga::colour!(White, Black);
    panic!();
}
