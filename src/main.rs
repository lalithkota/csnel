#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]
#![feature(llvm_asm)]

extern crate rlibc;

use core::panic::PanicInfo;

mod vga_buffer;
mod interrupts;
mod gdt;
mod pic_init;
mod pci;

fn init_interrupts(){
    gdt::init();
    interrupts::init_idt();
    unsafe { pic_init::PICS.lock().initialize()};
    x86_64::instructions::interrupts::enable();
}

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

    init_interrupts();

    pci::pci_init();
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
