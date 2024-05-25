#![no_std]
#![feature(abi_x86_interrupt)]

// extern crate alloc;

#[macro_use]
pub mod vga;
#[macro_use]
pub mod serial;
pub mod arch;
pub mod graphics;
pub mod logging;

pub fn init() {
    logging::init();
    arch::x86_64::gdt::init();
    arch::x86_64::interrupts::idt_load();
}
