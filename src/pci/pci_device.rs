pub struct PciDevice{
    pub bus_no : u8,
    pub slot_no : u8,
    pub vendor_id : u16,
    pub device_id : u16,
    pub header_type : u8,
}

impl PciDevice{
    pub const fn new() -> PciDevice{
        PciDevice { bus_no : 0xFF, slot_no : 0xFF, vendor_id : 0xFFFF, device_id : 0xFFFF, header_type : 0xFF}
    }
}

impl Copy for PciDevice { }

impl Clone for PciDevice {
    fn clone(&self) -> PciDevice {
        *self
    }
}
