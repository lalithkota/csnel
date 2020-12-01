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

pub fn init(boot_info:&'static BootInfo) -> (bool,crate::memory::OffsetPageTable<'static>){
	interrupts::init_interrupts();
	let mapper = unsafe{memory::init(boot_info.physical_memory_offset)};
	pci::pci_init();
	eth_driver::eth_driver_init(&mapper);
	net::init(&mapper);
	(true, mapper)
}
