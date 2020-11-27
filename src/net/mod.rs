pub mod head_struct;

use crate::eth_driver;
use crate::println;
use crate::print;
use x86_64;
use eth_driver::EthDriver;
use x86_64::instructions::port::Port;
use head_struct::EthernetHeader;
use eth_driver::ETH_DEV;

const ETH_HEADER_SIZE : usize = 14;


pub fn init(){
    loop{
        interrupt_poller();
        deal();
    }
}


fn deal(){
    let _rec_type = unsafe {&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8  as *const [u8;2])};
    unsafe{ETH_DEV.my_rx_ptr_offset+= 2};

    let _pack_length = unsafe {&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8  as *const [u8;2])};
    unsafe{ETH_DEV.my_rx_ptr_offset+=2};

    let packet_length = ((_pack_length[0] as u16) << 8) | (_pack_length[1] as u16);
    let length_with_ind = unsafe{ETH_DEV.my_rx_ptr_offset} + packet_length;

    let eth_header = unsafe {&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8  as *const EthernetHeader)};
    unsafe{ETH_DEV.my_rx_ptr_offset+= EthernetHeader::size_of()};

    if length_with_ind > 8192{

    }
    else{

    }
}


fn test_print(){
    let rb_start_temp : u32 = unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x30).read()};
    let capr_temp : u16 = unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x38).read()};
    let cbr_temp : u16 = unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x3A).read()};
    println!("RBSTART {:#x}",rb_start_temp);
    println!("CAPR {}",capr_temp as i16);
    println!("CBR {} and here is rx_buffer",cbr_temp);
    for i  in 0..cbr_temp as usize{
        unsafe{print!("{:#x}\t",eth_driver::ETH_DEV.rx_buffer[i]);}
    }
}


fn interrupt_poller(){
    loop{
        let temp1 : u16 = unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x3E).read()};
        if temp1 & 0b0100 != 0{
            println!("TOK INt received");
            unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x3E).write(temp1)};
            break;
        }
        if (temp1 & 1 != 0) && (temp1 & 0b1010000 != 0) {
            println!("Receive error");
            unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x3E).write(temp1)};
            break;
        }
        else if temp1 & 1 != 0 {
            println!("Receive INt received");
            unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x3E).write(temp1)};
            break;
        }
    }
}



pub fn test_transmit(){
    // const PACKET_SIZE : usize = 512;
    let src_mac = unsafe{eth_driver::ETH_DEV.get_mac_addr()};
    let mut packet  = EthernetHeader::new(src_mac , [0xFF as u8 ; 6], 0x0800 as u16);
    // let mut packet : [u8; PACKET_SIZE] = [0;PACKET_SIZE];
    let packet_addr : *mut EthernetHeader = &mut packet;
    // loop{
        unsafe{eth_driver::ETH_DEV.transmit_packet(packet_addr as u32,ETH_HEADER_SIZE)};
    // }

    // x86_64::instructions::interrupts::int43();
}
// #[repr(packed)]
// struct UdpHeader {
//   source_port: u16,
//   destination_port: u16,
//   length: u16,
//   crc: u16
// }
//
// impl UdpHeader {
//
//   fn new(source_port: u16, destination_port: u16, length: u16) -> UdpHeader {
//     UdpHeader {
//       source_port: source_port.to_be(),
//       destination_port: destination_port.to_be(),
//       length: (size_of::<UdpHeader>() as u16 + length).to_be(),
//       crc: 0
//     }
//   }
//
// }
//
// #[repr(packed)]
// struct IpHeader {
//   version_length: u8,
//   tos: u8,
//   length: u16,
//
//   id: [u8; 3],
//   flags_fragment: u8,
//
//   ttl: u8,
//   protocol: u8,
//   crc: u16,
//
//   source: u32,
//
//   destination: u32,
//
// }
//
// impl IpHeader {
//
//   fn new(payload_length: u16, protocol: u8, source: u32, destination: u32) -> IpHeader {
//     IpHeader {
//       version_length: ((0x4) << 4) | 5,
//       tos: 0,
//       length: size_of::<IpHeader>() as u16 + payload_length.to_be(),
//       id: [0, 0, 0],
//       flags_fragment: 0,
//       ttl: 30,
//       protocol: protocol,
//       source: source,
//       destination: destination,
//       crc: 0
//     }
//   }
//
// }
//
//
