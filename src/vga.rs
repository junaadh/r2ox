use core::{fmt, ops::Add};

use lazy_static::lazy_static;
use spin::Mutex;

pub const VGA_BUFFER: usize = 0xffff8000fd000000;
pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub(crate) fn new(forground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (forground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VgaChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
pub struct VgaBuffer {
    chars: [[volatile::Volatile<VgaChar>; VGA_WIDTH]; VGA_HEIGHT],
}

pub struct ImplWrite {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) color_code: ColorCode,
    pub(crate) buffer: &'static mut VgaBuffer,
}

impl ImplWrite {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\r' => self.x = 0,
            b => {
                if self.x > VGA_WIDTH {
                    self.new_line();
                }
                let row = self.y;
                let col = self.x;
                let color = self.color_code;

                self.buffer.chars[row][col].write(VgaChar {
                    ascii_character: b,
                    color_code: color,
                });
                self.x = self.x.add(1).min(VGA_WIDTH - 1);
            }
        }
    }

    pub fn write_str(&mut self, str: &str) {
        match str {
            "cls" => self.clear_screen(),
            s => s.chars().for_each(|c| self.write_byte(c as u8)),
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = VgaChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        (0..VGA_WIDTH).for_each(|col| {
            self.buffer.chars[row][col].write(blank);
        });
    }

    fn new_line(&mut self) {
        if self.y > VGA_HEIGHT {
            (1..VGA_HEIGHT).for_each(|row| {
                (0..VGA_WIDTH).for_each(|col| {
                    let char = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(char);
                })
            });
            self.y = VGA_HEIGHT - 1;
            self.clear_row(self.y);
            self.x = 0;
        } else {
            self.y += 1;
            self.x = 0;
        }
    }

    fn clear_screen(&mut self) {
        let char = VgaChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        (0..VGA_HEIGHT).for_each(|row| {
            (0..VGA_WIDTH).for_each(|col| {
                self.buffer.chars[row][col].write(char);
            })
        });
    }
}

impl fmt::Write for ImplWrite {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<ImplWrite> = Mutex::new(ImplWrite {
        x: 0,
        y: 0,
        color_code: ColorCode::new(Color::White, Color::Blue),
        buffer: unsafe { &mut *(VGA_BUFFER as *mut VgaBuffer) }
    });
}
#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => ($crate::vga::_vga_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! vga_println {
    () => ($crate::vga_print!("\n"));
    ($($arg:tt)*) => ($crate::vga_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _vga_print(args: fmt::Arguments) {
    use core::fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| WRITER.lock().write_fmt(args).unwrap());
}
