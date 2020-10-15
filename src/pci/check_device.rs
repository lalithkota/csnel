use super::base;
use super::pci_device;

pub fn check_device(bus : u8, slot : u8) -> (bool,pci_device::PciDevice){
    let vendor_id = base::get_vendor_id(bus, slot , 0);
    let device_id = base::get_device_id(bus, slot , 0);
    let header_type = base::get_header_type(bus, slot , 0);

    (vendor_id != 0xFFFF, pci_device::PciDevice{bus_no:bus, slot_no:slot, vendor_id, device_id, header_type})
}
