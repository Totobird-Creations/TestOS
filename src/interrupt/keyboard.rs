use x86_64::{
    structures::idt::{
        InterruptDescriptorTable,
        InterruptStackFrame
    },
    instructions::port::Port
};

use crate::tasks::task::keyboard::add_scancode;
use super::{
    PICS,
    InterruptIndex
};


const INDEX : InterruptIndex = InterruptIndex::Keyboard;


pub fn setup(idt : &mut InterruptDescriptorTable) {
    idt[INDEX.as_usize()].set_handler_fn(handler);
}

extern "x86-interrupt" fn handler(_stack_frame : InterruptStackFrame) {
    let mut port          = Port::new(0x60);
    let     scancode : u8 = unsafe {port.read()};
    
    add_scancode(scancode);

    unsafe {
        PICS.lock().notify_end_of_interrupt(INDEX.as_u8())
    }
}
