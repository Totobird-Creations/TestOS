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
    vga::colour!(LightRed, Black);
    vga::print!("EXCEPTION : DOUBLE FAULT {}\n{:#?}", error_code, stack_frame);
    vga::colour!(White, Black);
    panic!();
}
