use super::base::pci_config_read_word;
use super::pci_device;

pub fn check_device_and_add(bus : u8, slot : u8, pci_device : &mut pci_device::PciDevice) -> bool{
    let vendor_id = (pci_config_read_word(bus, slot, 0, 0) & 0xFFFF) as u16;//get_vendor_id(bus, slot , 0);
    let device_id = ((pci_config_read_word(bus, slot, 0, 0) & 0xFFFF0000) >> 16 ) as u16;//get_device_id(bus, slot , 0);
    let header_type = ((pci_config_read_word(bus, slot, 0, 0x0C) & 0x00FF0000) >> 16 ) as u8;//get_header_type(bus, slot , 0);

    if vendor_id != 0xFFFF {
        pci_device.vendor_id = vendor_id;
        pci_device.device_id = device_id;
        pci_device.header_type = header_type;
        pci_device.bus_no = bus;
        pci_device.slot_no = slot;
    }
    vendor_id != 0xFFFF
}

pub fn check_device(bus : u8, slot : u8) -> bool{
    let vendor_id = (pci_config_read_word(bus, slot, 0, 0) & 0xFFFF) as u16;//get_vendor_id(bus, slot , 0);
    let device_id = ((pci_config_read_word(bus, slot, 0, 0) & 0xFFFF0000) >> 16 ) as u16;//get_device_id(bus, slot , 0);
    let header_type = ((pci_config_read_word(bus, slot, 0, 0x0C) & 0x00FF0000) >> 16 ) as u8;//get_header_type(bus, slot , 0);

    vendor_id != 0xFFFF
}
