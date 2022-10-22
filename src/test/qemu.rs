pub use x86_64::instructions::{
    self,
    port::Port
};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}

pub fn exit(code : QemuExitCode) -> ! {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(code as u32);
    }
    loop {
        instructions::hlt();
    }
}