pub mod eth_driver_trait;
pub mod rtl8139_drivers;

use spin::Mutex;
use crate::println;
use crate::get_pci_dev_from_index;
pub use eth_driver_trait::EthDriver;
pub use rtl8139_drivers::RTL8139EthDriver;

pub static ETH_DEV: Mutex<RTL8139EthDriver> = Mutex::new(RTL8139EthDriver::new(0xFF));

pub fn init_eth_driver() -> bool{
    let (valid,eth_index) = detect_network_device();
    if valid{
        return load_network_device(eth_index);
    }
    else{
        println!("invalid");
		return false;
    }
}

pub fn detect_network_device() -> (bool,usize){
    // do nothing
    // for now lets hard code to 4th device
    (true,3)
}

pub fn load_network_device(eth_index : usize) -> bool {
    let eth_pci = get_pci_dev_from_index!(eth_index);

    if eth_pci.get_vendor_id()==0x10EC && eth_pci.get_device_id()==0x8139 {
        println!("Detected RTL");
        // unsafe{
		let eth_dev = &mut ETH_DEV.lock();
		eth_dev.pci_dev_no = eth_index;
		eth_dev.rb_start= eth_dev.get_rx_buffer_ptr_as_u64() as u32;
		eth_dev.load_driver();
        // }
		return true;
    }
    else {
        println!("Unknown Dev");
		return false;
    }
}
