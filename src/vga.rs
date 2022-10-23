use core::fmt::{
    self,
    Write
};

use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;
use alloc::string::String;

use crate::info::expect_panic;


pub const BUFFER_SIZE : (usize, usize) = (80, 25);

lazy_static! {
    pub static ref WRITER : Mutex<ScreenWriter> = Mutex::new(ScreenWriter {
        column : 0,
        colour : (Colour::White, Colour::Black),
        buffer : unsafe {&mut *(0xb8000 as *mut ScreenBuffer)},
    });
}


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    Black        = 0,
    Blue         = 1,
    Green        = 2,
    Cyan         = 3,
    Red          = 4,
    Magenta      = 5,
    Brown        = 6,
    LightGray    = 7,
    DarkGray     = 8,
    LightBlue    = 9,
    LightGreen   = 10,
    LightCyan    = 11,
    LightRed     = 12,
    LightMagenta = 13,
    Yellow       = 14,
    White        = 15
}
impl Colour {
    pub fn to_esc_fg(&self) -> &str {
        return match (self) {
            Colour::Black        => "\x1b[38;2;0;0;0m",
            Colour::Blue         => "\x1b[38;2;0;0;170m",
            Colour::Green        => "\x1b[38;2;0;170;0m",
            Colour::Cyan         => "\x1b[38;2;0;170;170m",
            Colour::Red          => "\x1b[38;2;170;0s;0m",
            Colour::Magenta      => "\x1b[38;2;170;0;170m",
            Colour::Brown        => "\x1b[38;2;170;85;0m",
            Colour::LightGray    => "\x1b[38;2;170;170;170m",
            Colour::DarkGray     => "\x1b[38;2;85;85;85m",
            Colour::LightBlue    => "\x1b[38;2;85;85;255m",
            Colour::LightGreen   => "\x1b[38;2;85;255;85m",
            Colour::LightCyan    => "\x1b[38;2;85;255;255m",
            Colour::LightRed     => "\x1b[38;2;255;85;85m",
            Colour::LightMagenta => "\x1b[38;2;255;85;255m",
            Colour::Yellow       => "\x1b[38;2;255;255;85m",
            Colour::White        => "\x1b[38;2;255;255;255m",
        }
    }
    pub fn to_esc_bg(&self) -> &str {
        return match (self) {
            Colour::Black        => "\x1b[48;2;0;0;0m",
            Colour::Blue         => "\x1b[48;2;0;0;170m",
            Colour::Green        => "\x1b[48;2;0;170;0m",
            Colour::Cyan         => "\x1b[48;2;0;170;170m",
            Colour::Red          => "\x1b[48;2;170;0;0m",
            Colour::Magenta      => "\x1b[48;2;170;0;170m",
            Colour::Brown        => "\x1b[48;2;170;85;0m",
            Colour::LightGray    => "\x1b[48;2;170;170;170m",
            Colour::DarkGray     => "\x1b[48;2;85;85;85m",
            Colour::LightBlue    => "\x1b[48;2;85;85;255m",
            Colour::LightGreen   => "\x1b[48;2;85;255;85m",
            Colour::LightCyan    => "\x1b[48;2;85;255;255m",
            Colour::LightRed     => "\x1b[48;2;255;85;85m",
            Colour::LightMagenta => "\x1b[48;2;255;85;255m",
            Colour::Yellow       => "\x1b[48;2;255;255;85m",
            Colour::White        => "\x1b[48;2;255;255;255m",
        }
    }
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
        column : usize,
    pub colour : (Colour, Colour),
        buffer : &'static mut ScreenBuffer,
}
#[allow(unused)]
impl ScreenWriter {
    fn write_byte(&mut self, byte : u8) {
        match (byte) {
            b'\n' => self.new_line(),
            byte  => {
                if (self.column >= BUFFER_SIZE.0) {
                    self.new_line();
                }

                let y = BUFFER_SIZE.1 - 1;
                let x = self.column;

                self.buffer.chars[y][x].write(BufferChar {
                    character : byte,
                    colour    : ColourCode::new(self.colour.0, self.colour.1),
                });
                self.column += 1;
            }
        }
    }
    fn _write_str(&mut self, s : &str) {
        for byte in s.bytes() {
            match (byte) {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _                   => self.write_byte(0xfe),
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
            colour    : ColourCode::new(self.colour.0, self.colour.1)
        };
        for x in 0..BUFFER_SIZE.0 {
            self.buffer.chars[y][x].write(blank);
        }
    }

    pub fn set_colour(&mut self, fg : Colour, bg : Colour) {
        self.colour = (fg, bg);
    }
}
impl fmt::Write for ScreenWriter {
    fn write_str(&mut self, s : &str) -> fmt::Result {
        self._write_str(s);
        return Ok(());
    }
}

pub struct Writable(String);
impl Writable {
    pub fn new() -> Writable {
        return Writable(String::new());
    }
    pub fn unwrap<'l>(self) -> String {
        return self.0;
    }
}
impl fmt::Write for Writable {
    fn write_str(&mut self, s : &str) -> fmt::Result {
        self.0 += s;
        return Ok(());
    }
}


pub macro println {
    () => {
        $crate::vga::println!("");
    },
    ($($arg:tt)*) => {{
        interrupts::without_interrupts(|| {
            let mut writable = Writable::new();
            writable.write_fmt(format_args!($($arg)*)).unwrap();
            let     written = writable.unwrap();
            let     lines   = written.split("\n");
            let mut writer  = WRITER.lock();
            let mut serial1 = crate::test::serial::SERIAL1.lock();
            for line in lines {

                write!(writer, "\n").unwrap();
                write!(writer, "{:width$}",
                    line, width=
                        if (line.len() >= BUFFER_SIZE.0) {
                            ((line.len() - 1) / (BUFFER_SIZE.0 + 1))
                        } else {
                            BUFFER_SIZE.0
                        }
                ).unwrap();

                if (expect_panic()) {
                    write!(serial1, "\n").unwrap();
                    write!(serial1, "{}{}{:width$}\x1b[0m",
                        writer.colour.0.to_esc_fg(),
                        writer.colour.1.to_esc_bg(),
                        line, width=
                            if (line.len() >= BUFFER_SIZE.0) {
                                ((line.len() - 1) / (BUFFER_SIZE.0 + 1))
                            } else {
                                BUFFER_SIZE.0
                            }
                    ).unwrap();
                }

            }
        });
    }}
}
pub macro print {
    ($($arg:tt)*) => {{
        interrupts::without_interrupts(|| {
            let mut writable = Writable::new();
            writable.write_fmt(format_args!($($arg)*)).unwrap();
            let     written = writable.unwrap();
            let mut writer  = WRITER.lock();
            let mut serial1 = crate::test::serial::SERIAL1.lock();

            write!(writer, "{}", written).unwrap();

            if (expect_panic()) {
                write!(serial1, "{}{}{}\x1b[0m",
                    writer.colour.0.to_esc_fg(),
                    writer.colour.1.to_esc_bg(),
                    written
                ).unwrap();
            }

        });
    }}
}
pub macro colour {
    () => {
        WRITER.lock().set_colour(Colour::White, Colour::Black);
    },
    ($fg:ident, $bg:ident) => {
        WRITER.lock().set_colour(Colour::$fg, Colour::$bg);
    }
}

pub macro warn {
    ($($arg:tt)*) => {
        colour!();
        colour!(Yellow, Black);
        print!("\n");
        print!("WARNING : ");
        print!($($arg)*);
        colour!();
    }
}
pub macro error {
    ($($arg:tt)*) => {
        colour!();
        colour!(LightRed, Black);
        print!("\n");
        print!("EXCEPTION : ");
        print!($($arg)*);
        colour!();
    }
}

pub macro format {
    ($($arg:tt)*) => ($crate::vga::_format(format_args!($($arg)*)))
}
#[doc(hidden)]
pub fn _format(args : fmt::Arguments) -> &str {
    return args.as_str().unwrap();
}
