#[repr(C)]
pub struct EthernetHeader {
  pub dest: [u8; 6],
  pub src: [u8; 6],
  pub eth_type: u16,
}

impl EthernetHeader {
  pub fn new(src: [u8; 6], dest: [u8; 6], eth_type: u16) -> EthernetHeader {
    EthernetHeader {
      dest: dest,
      src: src,
      eth_type: eth_type,
    }
  }

}
