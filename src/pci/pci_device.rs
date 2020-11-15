use super::base::pci_config_read_word;
use super::base::pci_config_write_word;

pub struct PciDevice{
    pub bus_no : u8,
    pub slot_no : u8,
    pub vendor_id : u16,
    pub device_id : u16,
    pub header_type : u8,
}

impl PciDevice{
    pub const fn new() -> PciDevice{
        PciDevice {
            bus_no : 0xFF,
            slot_no : 0xFF,
            vendor_id : 0xFFFF,
            device_id : 0xFFFF,
            header_type : 0xFF,
        }
    }
    pub fn get_vendor_id(&self) -> u16{
        // (pci_config_read_word(bus_no, slot_no, func, 0) & 0xFFFF) as u16
        self.vendor_id
    }
    pub fn get_device_id(&self) -> u16 {
        // ((pci_config_read_word(bus_no, slot_no, func, 0) & 0xFFFF0000) >> 16 ) as u16
        self.device_id
    }

    pub fn get_header_type(&self) -> u8 {
        // ((pci_config_read_word(bus_no, slot_no, func, 0x0C) & 0x00FF0000) >> 16 ) as u8
        self.header_type
    }

    pub fn get_status_reg(&self,func : u8) -> u16{
        ((pci_config_read_word(self.bus_no, self.slot_no, func, 0x04) & 0xFFFF0000) >> 16 ) as u16
    }

    pub fn get_command_reg(&self,func : u8) -> u16{
        (pci_config_read_word(self.bus_no, self.slot_no, func, 0x04) & 0xFFFF) as u16
    }

    pub fn enable_capabil_list(&self,func : u8){
        let mut tmp = pci_config_read_word(self.bus_no, self.slot_no, func, 0x04);
        tmp |= 1<<(4+16);
        pci_config_write_word(self.bus_no,self.slot_no,func,0x04,tmp);
    }

    pub fn get_cap_ptr(&self, func : u8) -> u8{
        (pci_config_read_word(self.bus_no, self.slot_no, func, 0x34) & 0x00FF) as u8
    }

    pub fn enable_bus_master(&self, func : u8){
        let mut tmp = pci_config_read_word(self.bus_no, self.slot_no, func, 0x04);
        tmp |= 1<<2;
        pci_config_write_word(self.bus_no,self.slot_no,func,0x04,tmp);
    }

    pub fn get_bar_with_no(&self, func : u8, base_addr_no : u8) -> u32{
        let ret : u32;
        if self.get_header_type() & 0x7F == 0x02 {
            ret = 0x0;
        }
        else {
            ret = pci_config_read_word(self.bus_no, self.slot_no, func, 0x10+0x04*base_addr_no);
        }
        ret
    }
    pub fn get_irq_pin_line(&self, func : u8) -> u16{
        (pci_config_read_word(self.bus_no, self.slot_no, func, 0x3C) & 0xFFFF) as u16
    }
}

impl Copy for PciDevice { }

impl Clone for PciDevice {
    fn clone(&self) -> PciDevice {
        *self
    }
}
