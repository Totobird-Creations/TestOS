use alloc::{format, string::String};


pub fn print(text : String) {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in text.as_bytes().iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2)     = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}
pub fn println(text : String) {
    print(text);
}
