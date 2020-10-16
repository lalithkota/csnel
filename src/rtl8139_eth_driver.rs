mod base;

use crate::println;
use x86_64::instructions::port::Port;

pub fn rtl8139_eth_driver_init(){
    let mut eth_dev = base::detect_network_device();
    base::enable_bus_master(&mut eth_dev);
    let base_addr = base::get_base_address(&mut eth_dev) as u16;
    println!("base_addr : {:#x}",base_addr);

    // get and print mac_addr
    let mac_addr_0_4 : u32 = unsafe { Port::new(base_addr).read()};
    let mac_addr_5_8 : u32 = unsafe { Port::new(base_addr+4).read()};
    println!("MAC addr : {:#x} - {:#x}",mac_addr_0_4 , mac_addr_5_8 & 0x0000FFFF);

    //
    unsafe{ Port::new(base_addr+0x52).write(0 as u8) }
}
