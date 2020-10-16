use super::base::*;

pub fn get_vendor_id(bus : u8, slot : u8, func : u8) -> u16 {
    (pci_config_read_word(bus, slot, func, 0) & 0xFFFF) as u16
}
pub fn get_device_id(bus : u8, slot : u8, func : u8) -> u16 {
    ((pci_config_read_word(bus, slot, func, 0) & 0xFFFF0000) >> 16 ) as u16
}

pub fn get_header_type(bus : u8, slot : u8, func : u8) -> u8 {
    ((pci_config_read_word(bus, slot, func, 0x0C) & 0x00FF0000) >> 16 ) as u8
}

pub fn get_status_reg(bus : u8, slot : u8, func : u8) -> u16{
    ((pci_config_read_word(bus, slot, func, 0x04) & 0xFFFF0000) >> 16 ) as u16
}

pub fn get_command_reg(bus : u8, slot : u8, func : u8) -> u16{
    (pci_config_read_word(bus, slot, func, 0x04) & 0xFFFF) as u16
}

pub fn enable_capabil_list(bus : u8, slot : u8, func : u8){
    let mut tmp = pci_config_read_word(bus, slot, func, 0x04);
    tmp |= 1<<(4+16);
    pci_config_write_word(bus,slot,func,0x04,tmp);
}
//
// pub fn get_cap_ptr(bus : u8, slot : u8, func : u8) -> u8{
//     (pci_config_read_word(bus, slot, func, 0x34) & 0x00FF) as u8
// }

pub fn enable_bus_master(bus : u8, slot : u8, func : u8){
    let mut tmp = pci_config_read_word(bus, slot, func, 0x04);
    tmp |= 1<<2;
    pci_config_write_word(bus,slot,func,0x04,tmp);
}

fn get_bar_with_no(bus : u8, slot : u8, func : u8, base_addr_no : u8) -> u64{
    pci_config_read_word(bus, slot, func, 0x10+0x04*base_addr_no)
}
pub fn get_base_addr(bus : u8, slot : u8, func : u8){
    get_bar_with_no(bus,slot,func,);
}
fn get_mmio_base_address(){

}
fn get_io_base_address(){

}
