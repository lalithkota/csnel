use crate::println;
use crate::pci;
use crate::pci::pci_device::PciDevice;

pub fn detect_network_device() -> PciDevice{
    // do nothing
    // for now lets hard code to 4th device
    pci::PCI_LIST.lock().list[3]
}

pub fn enable_bus_master(eth_dev : &mut PciDevice){
    println!("VID-DID:{:#x}-{:#x} ; Command reg : {:#x}",eth_dev.get_vendor_id(),eth_dev.get_device_id(),eth_dev.get_command_reg(0));
    eth_dev.enable_bus_master(0);
    println!("Command reg : {:#x}",eth_dev.get_command_reg(0));
}

pub fn get_base_address(eth_dev : &mut PciDevice) -> u32{
    let mut bar0 = eth_dev.get_bar_with_no(0,0);
    if bar0 & 0x1 == 0 {
        bar0=0x0;
    }
    bar0 & 0xFFFFFFFC
}

pub fn turn_on_eth_dev(eth_dev : &mut PciDevice){

}
