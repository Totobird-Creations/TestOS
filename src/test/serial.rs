use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;


lazy_static! {
    pub static ref SERIAL1 : Mutex<SerialPort> = {
        let mut port = unsafe {SerialPort::new(0x3f8)};
        port.init();
        Mutex::new(port)
    };
}
