use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame
};

use crate::vga;
use super::{
    PICS,
    InterruptIndex
};


const INDEX : InterruptIndex = InterruptIndex::Timer;


pub fn setup(idt : &mut InterruptDescriptorTable) {
    idt[INDEX.as_usize()].set_handler_fn(handler);
}

extern "x86-interrupt" fn handler(_stack_frame : InterruptStackFrame) {
    vga::print!(".");
    unsafe {
        PICS.lock().notify_end_of_interrupt(INDEX.as_u8())
    }
}
