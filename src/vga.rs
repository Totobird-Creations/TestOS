use core::fmt;

use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;


const BUFFER_SIZE : (usize, usize) = (80, 25);

lazy_static! {
    pub static ref WRITER : Mutex<ScreenWriter> = Mutex::new(ScreenWriter {
        column : 0,
        colour : ColourCode::new(Colour::White, Colour::Black),
        buffer : unsafe {&mut *(0xb8000 as *mut ScreenBuffer)},
    });
}


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColourCode(u8);
impl ColourCode {
    pub fn new (foreground : Colour, background : Colour) -> ColourCode {
        return ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct BufferChar {
    character : u8,
    colour    : ColourCode
}

#[repr(transparent)]
struct ScreenBuffer {
    chars : [[Volatile<BufferChar>; BUFFER_SIZE.0]; BUFFER_SIZE.1],
}

pub struct ScreenWriter {
    column  : usize,
    colour  : ColourCode,
    buffer  : &'static mut ScreenBuffer,
}
#[allow(unused)]
impl ScreenWriter {
    fn write_byte(&mut self, byte : u8) {
        match byte {
            b'\n' => self.new_line(),
            byte  => {
                if self.column >= BUFFER_SIZE.0 {
                    self.new_line();
                }

                let y = BUFFER_SIZE.1 - 1;
                let x = self.column;

                self.buffer.chars[y][x].write(BufferChar {
                    character : byte,
                    colour    : self.colour,
                });
                self.column += 1;
            }
        }
    }
    fn _write_str(&mut self, s : &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }

        }
    }

    fn new_line(&mut self) {
        for x in 0..BUFFER_SIZE.0 {
            for y in 1..BUFFER_SIZE.1 {
                let character = self.buffer.chars[y][x].read();
                self.buffer.chars[y - 1][x].write(character);
            }
        }
        self.clear_row(BUFFER_SIZE.1 - 1);
        self.column = 0;
    }
    fn clear_row(&mut self, y : usize) {
        let blank = BufferChar {
            character : b' ',
            colour    : self.colour
        };
        for x in 0..BUFFER_SIZE.0 {
            self.buffer.chars[y][x].write(blank);
        }
    }

    pub fn set_colour(&mut self, fg : Colour, bg : Colour) {
        self.colour = ColourCode::new(fg, bg);
    }
}
impl fmt::Write for ScreenWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self._write_str(s);
        Ok(())
    }
}


#[macro_export]
pub macro print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)))
}
#[doc(hidden)]
pub fn _print(args : fmt::Arguments) {
    use core::fmt::Write;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
pub macro colour {
    () => {
        WRITER.lock().set_colour(Colour::White, Colour::Black);
    },
    ($fg:ident, $bg:ident) => {
        WRITER.lock().set_colour(Colour::$fg, Colour::$bg);
    }
}
