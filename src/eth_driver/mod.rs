pub mod eth_driver_trait;
pub mod rtl8139_drivers;

use crate::println;
use crate::get_pci_dev_from_index;
pub use eth_driver_trait::EthDriver;
pub use rtl8139_drivers::RTL8139EthDriver;

pub static mut ETH_DEV: RTL8139EthDriver = RTL8139EthDriver::new(0xFF);

pub fn eth_driver_init(mapper : &crate::memory::OffsetPageTable<'static>){
    let (valid,eth_index) = detect_network_device();
    if valid{
        load_network_device(eth_index,mapper);
    }
    else{
        println!("invalid");
    }
}

pub fn detect_network_device() -> (bool,usize){
    // do nothing
    // for now lets hard code to 4th device
    (true,3)
}

pub fn load_network_device(eth_index : usize,mapper : &crate::memory::OffsetPageTable<'static>){
    let eth_pci = get_pci_dev_from_index!(eth_index);

    if eth_pci.get_vendor_id()==0x10EC && eth_pci.get_device_id()==0x8139 {
        println!("Detected RTL");
        unsafe{
            ETH_DEV.pci_dev_no = eth_index;
            ETH_DEV.rb_start= ETH_DEV.get_rx_buffer_ptr_as_u64(mapper) as u32;
            ETH_DEV.load_driver();
        }
    }
    else {
        println!("Unknown Dev");
    }
}
