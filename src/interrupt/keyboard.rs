use x86_64::{
    structures::idt::{
        InterruptDescriptorTable,
        InterruptStackFrame
    },
    instructions::port::Port
};
use spin::Mutex;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use lazy_static::lazy_static;

use crate::vga;
use super::{
    PICS,
    InterruptIndex
};


const INDEX : InterruptIndex = InterruptIndex::Keyboard;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1,
            HandleControl::Ignore)
        );
}


pub fn setup(idt : &mut InterruptDescriptorTable) {
    idt[INDEX.as_usize()].set_handler_fn(handler);
}

extern "x86-interrupt" fn handler(_stack_frame : InterruptStackFrame) {
    let mut keyboard      = KEYBOARD.lock();
    let mut port          = Port::new(0x60);
    let     scancode : u8 = unsafe {port.read()};
    
    if let Ok(Some(event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(event) {
            match (key) {
                DecodedKey::Unicode (character ) => vga::print!("{}", character),
                DecodedKey::RawKey  (key       ) => vga::print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(INDEX.as_u8())
    }
}
