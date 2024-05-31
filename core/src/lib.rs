#![no_std]
#![feature(abi_x86_interrupt, lang_items, error_in_core)]
#![allow(internal_features)]

extern crate alloc;

#[macro_use]
pub mod vga;
#[macro_use]
pub mod serial;
pub mod arch;
#[macro_use]
pub mod graphics;
pub mod logging;
pub mod memory;

pub fn init() {
    logging::init();
    arch::x86_64::gdt::init_boot();
    arch::x86_64::interrupts::idt_load();
    unsafe { arch::x86_64::interrupts::pic::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}
