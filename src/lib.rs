#![no_std]

#![feature(abi_x86_interrupt)]
#![feature(llvm_asm)]

extern crate rlibc;

pub mod vga;
pub mod interrupts;
pub mod pci;
pub mod eth_driver;
pub mod net;

use core::panic::PanicInfo;

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
