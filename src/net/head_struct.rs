#[repr(C)]
pub struct EthernetHeader {
    pub dest: [u8; 6],
    pub src: [u8; 6],
    pub eth_type: [u8;2],
}

impl EthernetHeader {
    pub fn new(src: [u8; 6], dest: [u8; 6], eth_type: u16) -> EthernetHeader {
        EthernetHeader {
            dest: dest,
            src: src,
            eth_type: [((eth_type >> 8)& 0xFF) as u8,(eth_type & 0xFF) as u8 ],
        }
    }
    pub fn print(&self){
    }

    pub fn size_of() -> u16{
        (6+6+2) as u16
    }
}
