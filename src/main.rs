#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // turn the screen gray
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        if let Some(x) = framebuffer.buffer_mut().first_mut() {
            *x = 0x80;
        }

    }

    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
