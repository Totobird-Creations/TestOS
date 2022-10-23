use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame
};

use crate::vga;


pub const DOUBLE_FAULT_IST_INDEX : u16 = 0;


pub fn setup(idt : &mut InterruptDescriptorTable) {
    unsafe {
        idt.double_fault.set_handler_fn(handler)
            .set_stack_index(DOUBLE_FAULT_IST_INDEX);
    }
}


extern "x86-interrupt" fn handler(stack_frame : InterruptStackFrame, error_code : u64) -> ! {
    vga::error!("DOUBLE FAULT {}", error_code);
    vga::colour!(LightRed, Black);
    vga::println!("{:#?}", stack_frame);
    vga::colour!();
    vga::print!("\n");
    panic!("A double fault error occured.");
}
