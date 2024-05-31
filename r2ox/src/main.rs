#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};
use libr2ox::{arch::x86_64::gdt, serial::SERIAL1, vga_println};
use spin::Once;
use x86_64::{
    instructions::{hlt, interrupts},
    VirtAddr,
};

// macro_rules! multiboot2 {
//     () => {
//         use core::arch::global_asm;
//         global_asm!(
//             "
//                     .section .boot
//                 header_start:
//                     .long 0xe85250d6                /* magic number (multiboot 2) */
//                     .long 0                         /* architecture 0 (protected mode i386) */
//                     .long header_end - header_start /* header length */
//                     .long 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start)) /* checksum */
//                     /* insert optional multiboot tags here */
//                     /* required end tag */
//                     .align 8
//                     .word 0    /* type */
//                     .word 0    /* flag */
//                     .word 8   /* size */
//                 header_end:

//                     .global start

//                     .section .bss
//                 stack_bottom:
//                     .zero 64
//                 stack_top:
//             "
//         );
//     };
// }

// .section .text
// .code32

// .section .bss
// .align 4

// multiboot2!();

const STACK_SIZE: u64 = 4096 * 4;
#[used]
static STACK: limine::request::StackSizeRequest =
    limine::request::StackSizeRequest::new().with_size(STACK_SIZE);
#[used]
static HHDM: limine::request::HhdmRequest = limine::request::HhdmRequest::new();
#[used]
static MEMMAP: limine::request::MemoryMapRequest = limine::request::MemoryMapRequest::new();

static PHY_OFFSET: Once<VirtAddr> = Once::new();

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    // no idt to handle interrupts
    interrupts::disable();

    STACK
        .get_response()
        .expect("Stack initialized check failed...");

    PHY_OFFSET.call_once(|| {
        VirtAddr::new(
            HHDM.get_response()
                .expect("Failed to get limine higher half direct mapping offset...")
                .offset(),
        )
    });

    let memmap = MEMMAP.get_response().expect("Fail to get memmap");

    gdt::init_boot();

    libr2ox::init();
    memmap.entries().iter().for_each(|x| {
        log::info!("base = {}, length = {}", x.base, x.length,);
    });
    vga_println!("hi");
    log::info!("booted");

    hcf()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(mut s) = SERIAL1.try_lock() {
        let _ = s.write_fmt(format_args!("{}\n", info));
    }
    hcf()
}

pub fn hcf() -> ! {
    loop {
        hlt();
        core::hint::spin_loop()
    }
}
