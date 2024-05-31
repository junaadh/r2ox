use lazy_static::lazy_static;
use log::warn;
use x86_64::{
    instructions::port::Port,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::println;

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(doublefault_handler)
                .set_stack_index(super::gdt::DOUBLE_FAULT_1ST_INDEX);
            idt[pic::InterruptIndex::Timer.as_u8()].set_handler_fn(timer_interrupt_handler);
            idt[pic::InterruptIndex::Keyboard.as_u8()].set_handler_fn(keyboard_interrupt_handler);
        }
        idt
    };
}

pub fn idt_load() {
    crate::arch::x86_64::interrupts::IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log::warn!("EXCEPTION: BREAKPOINT:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn doublefault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    log::error!(
        "EXCEPTION: DOUBLE FAULT:\n{:#?}\nERROR_CODE: {}",
        stack_frame,
        error_code
    );
    panic!("EXCEPTION: DOUBLE FAULT:\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    log::info!("{}", scancode);

    unsafe {
        pic::PICS
            .lock()
            .notify_end_of_interrupt(pic::InterruptIndex::Timer.as_u8())
    }
}
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    log::info!("-");
    log::warn!("-");
    warn!("{:#?}", _stack_frame);
    println!("-");
    unsafe {
        pic::PICS
            .lock()
            .notify_end_of_interrupt(pic::InterruptIndex::Keyboard.as_u8())
    }
}

pub mod pic {
    use pic8259::ChainedPics;
    use spin::Mutex;

    pub const PIC_1_OFFSET: u8 = 32;
    pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

    pub static PICS: Mutex<ChainedPics> =
        Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

    #[derive(Debug, Clone, Copy)]
    #[repr(u8)]
    pub enum InterruptIndex {
        Timer = PIC_1_OFFSET,
        Keyboard,
    }

    impl InterruptIndex {
        pub fn as_u8(self) -> u8 {
            self as u8
        }

        pub fn as_usize(self) -> usize {
            usize::from(self.as_u8())
        }
    }
}
