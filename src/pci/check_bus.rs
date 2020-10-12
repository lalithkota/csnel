use super::check_device;
use crate::println;

pub fn check_bus(bus : u8) {
    let mut un_used_slots = 0;
    for slot in 0..32 {
        if check_device::check_device(bus, slot as u8) {
            un_used_slots += 1;
        }
    }
    println!("In Bus{}, total no of un-used slots: {}  ", bus, un_used_slots);
}
