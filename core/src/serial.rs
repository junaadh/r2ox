use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

pub const SERIAL1_COM: u16 = 0x3f8;
pub const SERIAL2_COM: u16 = 0x2f8;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial = unsafe { SerialPort::new(SERIAL1_COM) };
        serial.init();
        Mutex::new(serial)
    };
    pub static ref SERIAL2: Mutex<SerialPort> = {
        let mut serial = unsafe { SerialPort::new(SERIAL2_COM) };
        serial.init();
        Mutex::new(serial)
    };
}

#[doc(hidden)]
pub fn _print1(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}
#[doc(hidden)]
pub fn _print2(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL2
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

#[macro_export]
macro_rules! serial1_print {
    ($($args:tt)*) => {
        $crate::serial::_print1(format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! serial1_println {
    () => {
        $crate::serial1_print!("\n")
    };
    ($fmt:expr) => {
        $crate::serial1_print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => ($crate::serial1_print!(concat!($fmt, "\n"), $($arg)*))
}

#[macro_export]
macro_rules! serial2_print {
    ($($args:tt)*) => {
        $crate::serial::_print2(format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! serial2_println {
    () => {
        $crate::serial2_print!("\n")
    };
    ($fmt:expr) => {
        $crate::serial2_print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => ($crate::serial2_print!(concat!($fmt, "\n"), $($arg)*))
}
