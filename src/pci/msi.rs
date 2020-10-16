use super::base;
use super::PCI_LIST;
use crate::println;

pub fn print_sta_com(){
    for i in 0..32 {
        let eth_dev = PCI_LIST.lock().list[i];
        if eth_dev.get_vendor_id()==0xFFFF{
            break;
        }
        println!("VID-DID : {:#x}-{:#x}",eth_dev.get_vendor_id(),eth_dev.get_device_id());
        println!("Status reg: {:#x}; Command Reg: {:#x}",eth_dev.get_status_reg(0),eth_dev.get_command_reg(0));
        eth_dev.enable_capabil_list(0);
        println!("Capbilities Ptr : {:#x}",eth_dev.get_cap_ptr(0));
        println!("Status reg: {:#x}; Command Reg: {:#x}",eth_dev.get_status_reg(0),eth_dev.get_command_reg(0));
    }
}
pub fn enable_msi(){
    let eth_dev = PCI_LIST.lock().list[3];
    println!("VID-DID : {:#x}-{:#x}",eth_dev.get_vendor_id(),eth_dev.get_device_id());
    println!("Status reg: {:#x}; Command Reg: {:#x}",eth_dev.get_status_reg(0),eth_dev.get_command_reg(0));
    eth_dev.enable_capabil_list(0);
    println!("Capbilities Ptr : {:#x}",eth_dev.get_cap_ptr(0));
    println!("Status reg: {:#x}; Command Reg: {:#x}",eth_dev.get_status_reg(0),eth_dev.get_command_reg(0));
    let cap_ptr = eth_dev.get_cap_ptr(0);
    println!("cap_ptr address' value : {:#x}",base::pci_config_read_word(eth_dev.bus_no,eth_dev.slot_no,0,cap_ptr));
}
