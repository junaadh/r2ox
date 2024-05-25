use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(doublefault_handler)
                .set_stack_index(super::gdt::DOUBLE_FAULT_1ST_INDEX);
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
    panic!("EXCEPTION: DOUBLE FAULT:\n{:#?}", stack_frame,)
}
