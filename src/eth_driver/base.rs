use crate::println;
use crate::pci;

pub fn detect_network_device(){
    // do nothing
    // for now lets hard code to 4th device
}

pub fn enable_bus_master(){
    detect_network_device();
    let eth_dev = unsafe{pci::PCI_LIST[3]};
    println!("VID-DID:{:#x}-{:#x} ; Command reg : {:#x}",pci::base::get_vendor_id(eth_dev.bus_no,eth_dev.slot_no,0),
        pci::base::get_device_id(eth_dev.bus_no,eth_dev.slot_no,0),
        pci::base::get_command_reg(eth_dev.bus_no,eth_dev.slot_no,0));
    pci::base::enable_bus_master(eth_dev.bus_no,eth_dev.slot_no,0);
    println!("Command reg : {:#x}",pci::base::get_command_reg(eth_dev.bus_no,eth_dev.slot_no,0));
}
