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
    vga::error!("PAGE FAULT - {:?}", error_code);
    vga::error!("ADDRESS    - {:?}", Cr2::read());
    vga::colour!(LightRed, Black);
    vga::println!("{:#?}", stack_frame);
    vga::colour!();
    vga::print!("\n");
    panic!("A page fault error occured.");
}
