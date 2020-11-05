#![no_std]

#![feature(abi_x86_interrupt)]
#![feature(llvm_asm)]

extern crate rlibc;

pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod pic_init;
pub mod pci;
pub mod eth_driver;

use core::panic::PanicInfo;

pub fn init_interrupts(){
    gdt::init();
    interrupts::init_idt();
    unsafe { pic_init::PICS.lock().initialize()};
    x86_64::instructions::interrupts::enable();
}

/// This function is called on panic.
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
