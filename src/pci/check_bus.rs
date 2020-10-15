use super::check_device;
use crate::println;
use super::PCI_LIST;

pub fn check_bus(bus : u8) {
    let mut count = 0;
    for slot in 0..32 {
        let (valid, pci_device) = check_device::check_device(bus, slot as u8);
        if valid {
            unsafe {PCI_LIST[count] = pci_device;}
            count += 1;
        }
    }
    println!("In Bus{}, total no of un-used slots: {}  ", bus, 32 - count);
}
