use super::base;
use super::PCI_LIST;
use crate::println;

pub fn _print_sta_com(){
    for i in 0..32 {
        let eth_dev = unsafe {PCI_LIST[i]};
        if base::get_vendor_id(eth_dev.bus_no,eth_dev.slot_no,0)==0xFFFF{
            break;
        }
        println!("VID-DID : {:#x}-{:#x}",base::get_vendor_id(eth_dev.bus_no,eth_dev.slot_no,0),base::get_device_id(eth_dev.bus_no,eth_dev.slot_no,0));
        println!("Status reg: {:#x}; Command Reg: {:#x}",base::get_status_reg(eth_dev.bus_no,eth_dev.slot_no,0),base::get_command_reg(eth_dev.bus_no,eth_dev.slot_no,0));
        base::enable_capabil_list(eth_dev.bus_no,eth_dev.slot_no,0);
        println!("Capbilities Ptr : {:#x}",base::pci_config_read_word(eth_dev.bus_no,eth_dev.slot_no,0,0x34));
        println!("Status reg: {:#x}; Command Reg: {:#x}",base::get_status_reg(eth_dev.bus_no,eth_dev.slot_no,0),base::get_command_reg(eth_dev.bus_no,eth_dev.slot_no,0));
    }
}
pub fn _enable_msi(){
    let eth_dev = unsafe {PCI_LIST[3]};
    println!("VID-DID : {:#x}-{:#x}",base::get_vendor_id(eth_dev.bus_no,eth_dev.slot_no,0),base::get_device_id(eth_dev.bus_no,eth_dev.slot_no,0));
    println!("Status reg: {:#x}; Command Reg: {:#x}",base::get_status_reg(eth_dev.bus_no,eth_dev.slot_no,0),base::get_command_reg(eth_dev.bus_no,eth_dev.slot_no,0));
    base::enable_capabil_list(eth_dev.bus_no,eth_dev.slot_no,0);
    println!("Capbilities Ptr : {:#x}",base::pci_config_read_word(eth_dev.bus_no,eth_dev.slot_no,0,0x34));
    println!("Status reg: {:#x}; Command Reg: {:#x}",base::get_status_reg(eth_dev.bus_no,eth_dev.slot_no,0),base::get_command_reg(eth_dev.bus_no,eth_dev.slot_no,0));
    let cap_ptr = (base::pci_config_read_word(eth_dev.bus_no,eth_dev.slot_no,0,0x34) & 0xFF) as u8;
    println!("cap_ptr address' value : {:#x}",base::pci_config_read_word(eth_dev.bus_no,eth_dev.slot_no,0,cap_ptr));
}
