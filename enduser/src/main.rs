#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]

extern crate csnel;

mod mymain;

csnel::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static csnel::BootInfo) -> !{
    mymain::starter(boot_info);
    csnel::hlt_loop();
}
