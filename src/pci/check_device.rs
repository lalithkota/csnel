use super::base;
use crate::println;

pub fn check_device(bus : u8, slot : u8) -> bool{
    let vendor_id = base::get_vendor_id(bus, slot , 0);
    let device_id = base::get_device_id(bus, slot , 0);
    let header_type;

    if vendor_id != 0xFFFF {
        println!("New Device at slot:{} ; bus:{} ; VID-DID:{:#x}-{:#x}",slot,bus,vendor_id,device_id);
        header_type = base::get_header_type(bus,slot,0);
        if (header_type & 0x80) == 1 {
            println!("Multi Function device");
        }

        if (header_type & 0x7F) == 0x00 {
            println!("Standard Device");
        }
        else if (header_type & 0x7F) == 0x01 {
            println!("PCI-PCI Bridge Device");
        }
        else if (header_type & 0x7F) == 0x02 {
            println!("CardBus Bridge Device");
        }
    }

    vendor_id == 0xFFFF
}
