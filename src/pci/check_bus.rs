use super::check_device;
use crate::println;
use super::pci_device;

pub fn check_bus_and_get_list(bus : u8, pci_list : &mut [pci_device::PciDevice; 32],init_count : u8) -> u8 {
    let mut count = 0;
    for slot in 0..32 {
        if check_device::check_device_and_add(bus, slot as u8, &mut pci_list[init_count as usize + count]) {
            count += 1;
        }
    }
    println!("In Bus{}, total no of un-used slots: {}  ", bus, 32 - count);
    count as u8
}

pub fn check_bus(bus : u8) -> u8 {
    let mut count = 0;
    for slot in 0..32 {
        if check_device::check_device(bus, slot as u8) {
            // unsafe {PCI_LIST[count] = pci_device;}
            count += 1;
        }
    }
    println!("In Bus{}, total no of un-used slots: {}  ", bus, 32 - count);
    count as u8
}
