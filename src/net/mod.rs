pub mod head_struct;
pub mod arp;
pub mod ip;
pub mod tcp;

use crate::println;
use crate::print;

use x86_64::instructions::port::Port;

use crate::eth_driver;
use eth_driver::EthDriver;
use eth_driver::ETH_DEV;

use crate::memory::OffsetPageTable;
// use crate::memory::MapperAllSizes;

pub use head_struct::EthernetHeader;
pub use head_struct::ARPHeader;
pub use head_struct::IPHeader;
pub use head_struct::TCPHeader;

pub fn init(mapper : &OffsetPageTable<'static>){
    loop{
        let interr = interrupt_poller();
        if interr==2 {
			deal(mapper);
		}
    }
}


fn deal(mapper : &OffsetPageTable<'static>){
    let _rec_type = unsafe {&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8  as *const [u8;2])};
    unsafe{ETH_DEV.my_rx_ptr_offset+= 2};

    let _pack_length = unsafe {&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8  as *const [u8;2])};
    unsafe{ETH_DEV.my_rx_ptr_offset+=2};

    let packet_length = ((_pack_length[1] as u16) << 8) | (_pack_length[0] as u16);
    let mut length_with_ind = unsafe{ETH_DEV.my_rx_ptr_offset} + packet_length;

    let eth_header = unsafe {&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8  as *const EthernetHeader)};
    unsafe{ETH_DEV.my_rx_ptr_offset+= EthernetHeader::size_of()};

	if eth_header.get_type() == 0x806 { // arp packet(or)request

		let arp_req_header = unsafe {&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8  as *const ARPHeader)};
		unsafe{ETH_DEV.my_rx_ptr_offset+= ARPHeader::size_of()};

		arp::arp_deal(arp_req_header, eth_header, mapper);
        //tempo fix
        //length_with_ind = unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x3A).read()};
	}
	else if eth_header.get_type() == 0x800 { // ip packet

		let ip_header = unsafe {&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8  as *const IPHeader)};
		unsafe{ETH_DEV.my_rx_ptr_offset+= IPHeader::size_of()};

		if ip_header.is_tcp() {
			let tcp_header = unsafe {&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8  as *const TCPHeader)};
			unsafe{ETH_DEV.my_rx_ptr_offset+= TCPHeader::size_of()};

			if tcp_header.is_ack() & tcp_header.is_syn() {
				println!("TCP: Thats weird");
			}
			else if tcp_header.is_syn() & (!tcp_header.is_ack()){

				if tcp_header.get_data_off() > 5{
					println!("tcp: Options Received");
					let option_size = (tcp_header.get_data_off() as u16 - 5)*4;
					let options = unsafe {&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8  as *const [u8;40])};
					unsafe{ETH_DEV.my_rx_ptr_offset+= option_size};
                    println!("data off is {}",tcp_header.get_data_off());
					tcp::tcp_opt_deal(eth_header, ip_header, tcp_header, options, option_size, mapper);
				}
				else if tcp_header.get_data_off() == 5 {
					let option_size = 0;
					let options : &[u8;40] = &[0 as u8;40];
					tcp::tcp_opt_deal(eth_header, ip_header, tcp_header, options, option_size, mapper);
				}
			}
			else if (!tcp_header.is_syn()) & tcp_header.is_ack(){
				tcp::tcp_deal(eth_header, ip_header, tcp_header, mapper);
			}
		}
		else{
			println!("Some other ip packet received");
		}

	}
	else{
		println!("some other ether packet received");
	}

	if length_with_ind & 3 != 0{
		length_with_ind &= !(3);
		length_with_ind += 4;
	}

    if length_with_ind > 8192{ // TODO: there might be a serious bug here...
		unsafe{ETH_DEV.my_rx_ptr_offset = 0};
		unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x38).write(0xFFF0 as u16)};
		unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x3A).write(0x0 as u16)};
    }
    else{
		unsafe{ETH_DEV.my_rx_ptr_offset = length_with_ind};
		unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x38).write(ETH_DEV.my_rx_ptr_offset)};
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


fn interrupt_poller() -> i8{
    loop{
        let temp1 : u16 = unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x3E).read()};
        if temp1 & 0b0100 != 0{
            println!("TOK INt received");
            unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x3E).write(0b100 as u16)};
            return 1;
        }
        else if (temp1 & 1 != 0) && (temp1 & 0b1010000 != 0) {
            println!("Receive error");
            unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x3E).write(0b1010001 as u16)};
            return -2;
        }
        else if temp1 & 0b1 != 0 {
            println!("Receive INt received");
            unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x3E).write(0b1 as u16)};
            return 2;
        }
		else if temp1 != 0{
			println!("some other interrupts {:#x}",temp1);
			unsafe{Port::new(eth_driver::ETH_DEV.base_addr + 0x3E).write(temp1)};
			return -3;
		}
		else{
			// loop off
		}
    }
}



pub fn test_transmit(){
	const ETH_HEADER_SIZE : usize = 14;
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
