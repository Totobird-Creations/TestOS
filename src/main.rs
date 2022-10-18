// Following tutorial `https://os.phil-opp.com`
//
// Build with `cargo bootimage`

#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Freeze and do nothing on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in b"Hello World!".iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2)     = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
