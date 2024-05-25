#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};

use limine::request::FramebufferRequest;
use serial::SERIAL1;
use x86_64::instructions::hlt;

#[macro_use]
pub mod vga;
#[macro_use]
pub mod serial;
pub mod logging;

static FBR: FramebufferRequest = FramebufferRequest::new();

#[no_mangle]
pub extern "C" fn start() -> ! {
    logging::init();

    let fb = if let Some(fb) = FBR.get_response() {
        fb
    } else {
        panic!("ohno")
    };
    let fb = fb.framebuffers().last().unwrap();
    log::info!(
        "{:#?} - {:#?} - {:#?} - {:#?}",
        fb.addr(),
        fb.width(),
        fb.height(),
        fb.bpp()
    );

    let color = 0x696969; // Red in RGB
    for y in 0..fb.height() {
        for x in 0..fb.width() {
            unsafe {
                *(fb.addr() as *mut u32).add(y as usize * fb.pitch() as usize / 4 + x as usize) =
                    color;
            }
        }
    }

    log::warn!("damnit");
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
