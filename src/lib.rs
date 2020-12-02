#![no_std]

#![feature(abi_x86_interrupt)]

extern crate rlibc;

pub mod vga;
pub mod interrupts;
pub mod pci;
pub mod eth_driver;
pub mod net;
pub mod memory;

use core::panic::PanicInfo;

pub use bootloader::entry_point;
pub use bootloader::BootInfo;

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

pub fn init(boot_info:&'static BootInfo) -> (bool,memory::OffsetPageTable<'static>){
	loop{
		while interrupts::init_interrupts() {
			let mapper = memory::init_memory(boot_info.physical_memory_offset);
			unsafe{crate::memory::MAPPER_PTR = &mapper as *const crate::memory::OffsetPageTable<'static> as u64};
			while pci::init_pci() {
				while eth_driver::init_eth_driver() {
					while net::init_net() {
						return (true,mapper);
					}
				}
			}
		}
	}
}
