use crate::println;
use super::PCI_LIST;

pub fn check_pci_list(){
    for i in 0..32 {

        let pci_dev = unsafe{PCI_LIST[i]};

        if pci_dev.vendor_id == 0xFFFF {
            break;
        }
        println!("New Device at slot:{} ; bus:{} ; VID-DID:{:#x}-{:#x}", pci_dev.slot_no, pci_dev.bus_no, pci_dev.vendor_id, pci_dev.device_id);
        if (pci_dev.header_type & 0x80) == 1 {
            println!("Multi Function device");
        }

        if (pci_dev.header_type & 0x7F) == 0x00 {
            println!("Standard Device");
        }
        else if (pci_dev.header_type & 0x7F) == 0x01 {
            println!("PCI-PCI Bridge Device");
        }
        else if (pci_dev.header_type & 0x7F) == 0x02 {
            println!("CardBus Bridge Device");
        }
    }
}
