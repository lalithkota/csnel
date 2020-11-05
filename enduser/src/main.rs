#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]
#![feature(llvm_asm)]

extern crate csnel;

mod mymain;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    mymain::starter();
    csnel::hlt_loop();
}
