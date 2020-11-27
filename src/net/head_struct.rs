#[repr(C)]
struct EthernetHeader {
  dest: [u8; 6],
  src: [u8; 6],
  eth_type: u16,
}

impl EthernetHeader {
  fn new(src: [u8; 6], dest: [u8; 6], eth_type: u16) -> EthernetHeader {
    EthernetHeader {
      dest: dest,
      src: src,
      eth_type: eth_type,
    }
  }

}
