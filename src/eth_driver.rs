mod eth_driver_trait;
mod rtl8139_drivers;

use crate::println;
use crate::get_pci_dev_from_index;
use eth_driver_trait::EthDriver;
use rtl8139_drivers::RTL8139EthDriver;

pub fn eth_driver_init(){
    detect_and_load_network_device();
}

pub fn detect_and_load_network_device(){
    // do nothing
    // for now lets hard code to 4th device
    let eth_index = 3;
    let eth_pci = get_pci_dev_from_index!(eth_index);

    if eth_pci.get_vendor_id()==0x10EC && eth_pci.get_device_id()==0x8139 {
        println!("Detected RTL");
        let mut eth_dev = RTL8139EthDriver::new(eth_index);
        eth_dev.load_driver();
    }
    else {
        println!("Unknown Dev");
    }
}
