#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]
#![feature(llvm_asm)]

extern crate rlibc;

mod vga;
mod interrupts;
mod pci;
mod eth_driver;
mod net;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    interrupts::init_interrupts();

    pci::pci_init();
    eth_driver::eth_driver_init();
    net::init();
    // loop {
    //     use crate::print;
    //     print!("-");
    // }
    // fn stack_overflow(){
    //     stack_overflow();
    // }
    // stack_overflow();
    //
    // x86_64::instructions::interrupts::int3();

    println!("It did not crash!");
    hlt_loop();
}
