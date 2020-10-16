use x86_64::instructions::port::Port;

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

    unsafe { Port::new(0xCF8).write(address) };

    // tmp = (io_read_32(0xCFC) >> ((offset & 2) * 8)) & 0xffff) as u16;
    // tmp
    let ret : u32 = unsafe { Port::new(0xCFC).read() };
    ret
}

pub fn pci_config_write_word ( bus : u8, slot : u8, func : u8, offset : u8, write_word : u32){
    let address : u32;
    let lbus = bus as u32;
    let lslot = slot as u32;
    let lfunc = func as u32;
    let floor_offset = offset & 0xFC;
    address = ((lbus << 16) | (lslot << 11) | (lfunc << 8) | floor_offset as u32 | (0x80000000 as u32)) as u32;
    unsafe { Port::new(0xCF8).write(address) };
    unsafe { Port::new(0xCFC).write(write_word) };
}
