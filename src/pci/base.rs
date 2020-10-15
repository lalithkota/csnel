#[inline]
fn io_read_32(addr : u16) -> u32 {
    let mut ret : u32;
    unsafe{
        llvm_asm!("inl %dx, %eax" : "={ax}"(ret) : "{dx}"(addr) :: "volatile");
    }
    ret
}

#[inline]
fn io_write_32(addr : u16, value : u32){
    unsafe{
        llvm_asm!("outl %eax, %dx" :: "{dx}"(addr), "{al}"(value));
    }
}

//uint16_t pciConfigReadWord (uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
pub fn pci_config_read_word ( bus : u8, slot : u8, func : u8, offset : u8) -> u32 {
    let address : u32;
    // let tmp : u16;

    //uint32_t
    let lbus = bus as u32;//(uint32_t)bus;
    //uint32_t
    let lslot = slot as u32;//(uint32_t)slot;
    //uint32_t
    let lfunc = func as u32;//(uint32_t)func;

    let floor_offset = offset & 0xFC; // this is to floor the offset to its nearest 4 multiple

    //address = (uint32_t)((lbus << 16) | (lslot << 11) | (lfunc << 8) | (offset & 0xfc) | ((uint32_t)0x80000000));
    address = ((lbus << 16) | (lslot << 11) | (lfunc << 8) | floor_offset as u32 | (0x80000000 as u32)) as u32;

    io_write_32(0xCF8, address);

    // tmp = (io_read_32(0xCFC) >> ((offset & 2) * 8)) & 0xffff) as u16;
    // tmp
    io_read_32(0xCFC)
}

pub fn pci_config_write_word ( bus : u8, slot : u8, func : u8, offset : u8, write_word : u32){
    let address : u32;
    let lbus = bus as u32;
    let lslot = slot as u32;
    let lfunc = func as u32;
    let floor_offset = offset & 0xFC;
    address = ((lbus << 16) | (lslot << 11) | (lfunc << 8) | floor_offset as u32 | (0x80000000 as u32)) as u32;
    io_write_32(0xCF8, address);
    io_write_32(0xCFC, write_word);
}

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
