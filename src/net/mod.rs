pub mod head_struct;
pub mod arp;
pub mod ip;
pub mod tcp;
pub mod temp_http_response;

use crate::println;
use crate::print;

use x86_64::instructions::port::Port;

use crate::eth_driver;
use eth_driver::EthDriver;
use eth_driver::ETH_DEV;

pub use head_struct::EthernetHeader;
pub use head_struct::ARPHeader;
pub use head_struct::IPHeader;
pub use head_struct::TCPHeader;

pub fn init_net() -> bool{
	// using the regular interrupt handling .... lets hard code 11th handler for now ... rtl uses that
	// then unmask the interrupt line 
	
	crate::interrupts::IRQ_HANDLERS.lock()[11] = interrupt_handler;
	unsafe{Port::new(0xA1).write(Port::<u8>::new(0xA1).read() & !(1<<(11-8)));}
	// sample_deal_all();
	
	// do any other init .. like tcp buffer,etc :TODO
	
	true
}

fn interrupt_handler(stack_frame : &mut crate::interrupts::InterruptStackFrame){
	// not able to deal with the packet directly inside the interrupt handler..
	// getting page fault while converting virtaddr to physaddr inside net::transmit_packet()
	// instead we can poll for intr .. and deal accordingly
	// so will have to keep the intr masked... 
	// so use sample_deal_all() instead of this
	let interr = interrupt_asker();
	if interr ==2 {
		deal_packet();
		// println!("rec intr");
	}
}

pub fn sample_deal_all(){
	loop {
		let interr = interrupt_asker();
		if interr ==2 {
			deal_packet();
		}
	}
}

fn deal_packet(){
    let _rec_type = unsafe {&*(ETH_DEV.lock().get_from_and_update_buffer(2) as *const [u8;2])};

    let _pack_length = unsafe {&*(ETH_DEV.lock().get_from_and_update_buffer(2) as *const [u8;2])};

    let packet_length = ((_pack_length[1] as u16) << 8) | (_pack_length[0] as u16);
    let mut length_with_ind = ETH_DEV.lock().my_rx_ptr_offset + packet_length;

    let eth_header = unsafe {&*(ETH_DEV.lock().get_from_and_update_buffer(EthernetHeader::size_of()) as *const EthernetHeader)};

	if eth_header.get_type() == 0x806 { // arp packet(or)request

		let arp_req_header = unsafe {&*(ETH_DEV.lock().get_from_and_update_buffer(ARPHeader::size_of()) as *const ARPHeader)};
		
		arp::arp_deal(arp_req_header, eth_header);
	}
	else if eth_header.get_type() == 0x800 { // ip packet
		let ip_header = unsafe {&*(ETH_DEV.lock().get_from_and_update_buffer(IPHeader::size_of()) as *const IPHeader)};

		if ip_header.is_tcp() {
			let tcp_header = unsafe {&*(ETH_DEV.lock().get_from_and_update_buffer(TCPHeader::size_of()) as *const TCPHeader)};

			if tcp_header.is_ack() & tcp_header.is_syn() {
				println!("TCP: Thats weird");
			}
			else if tcp_header.is_syn() & (!tcp_header.is_ack()){

				if tcp_header.get_data_off() > 5{
					println!("tcp: Options Received");
					
					let option_size = (tcp_header.get_data_off() as u16 - 5)*4;
					
					let options = unsafe {&*(ETH_DEV.lock().get_from_and_update_buffer(option_size) as *const [u8;40])};
					
                    println!("data off is {}",tcp_header.get_data_off());
					
					tcp::tcp_opt_deal(eth_header, ip_header, tcp_header, options, option_size);
				}
				else if tcp_header.get_data_off() == 5 {
					let option_size = 0;
					let options : &[u8;40] = &[0 as u8;40];
					
					tcp::tcp_opt_deal(eth_header, ip_header, tcp_header, options, option_size);
				}
			}
			else if !tcp_header.is_syn(){
				if tcp_header.is_ack(){
					// println!("ACK Received");
				}
				tcp::tcp_deal(eth_header, ip_header, tcp_header);
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
		ETH_DEV.lock().my_rx_ptr_offset = 0;
		unsafe{Port::new(ETH_DEV.lock().base_addr + 0x38).write(0xFFF0 as u16)};
		unsafe{Port::new(ETH_DEV.lock().base_addr + 0x3A).write(0x0 as u16)};
    }
    else{
		ETH_DEV.lock().my_rx_ptr_offset = length_with_ind;
		let mut capr_port = Port::new(ETH_DEV.lock().base_addr + 0x38);
		unsafe{capr_port.write(ETH_DEV.lock().my_rx_ptr_offset - 16)};
    }
}

pub fn transmit_packet(packet_ref : u64, packet_len : usize) -> bool{
	use x86_64::VirtAddr;
	use crate::memory::MapperAllSizes;
	// let mapper = unsafe{&*(*MAPPER_ADDR.lock() as *const crate::memory::OffsetPageTable<'static>)};
	let mapper = unsafe{&*(crate::memory::MAPPER_PTR as *const crate::memory::OffsetPageTable<'static>)};
	let phys = mapper.translate_addr(VirtAddr::new(packet_ref));
	ETH_DEV.lock().transmit_packet(phys.unwrap().as_u64() as u32, packet_len);
	while interrupt_asker()!=0 && interrupt_asker()!=1 {}
	true
}

fn interrupt_asker() -> i8{
	let eth_dev = &mut ETH_DEV.lock();
	let temp1 : u16 = unsafe{Port::new(eth_dev.base_addr + 0x3E).read()};
	if temp1 & 0b0100 != 0{
		println!("TOK INt received");
		unsafe{Port::new(eth_dev.base_addr + 0x3E).write(0b100 as u16)};
		return 1;
	}
	else if (temp1 & 1 != 0) && (temp1 & 0b1010000 != 0) {
		println!("Receive error");
		unsafe{Port::new(eth_dev.base_addr + 0x3E).write(0b1010001 as u16)};
		return -2;
	}
	else if temp1 & 0b1 != 0 {
		println!("Receive INt received");
		unsafe{Port::new(eth_dev.base_addr + 0x3E).write(0b1 as u16)};
		return 2;
	}
	else if temp1 != 0{
		println!("some other interrupts {:#x}",temp1);
		if temp1 & 0b10000 != 0{ // rx buffer overflow ... 
			let rb_start_temp : u32 = unsafe{Port::new(eth_dev.base_addr + 0x30).read()};
			let capr_temp : u16 = unsafe{Port::new(eth_dev.base_addr + 0x38).read()};
			let cbr_temp : u16 = unsafe{Port::new(eth_dev.base_addr + 0x3A).read()};
			println!("RBSTART {:#x}",rb_start_temp);
			println!("CAPR {}",capr_temp as i16);
			println!("CBR {}",cbr_temp);
		}
		
		unsafe{Port::new(eth_dev.base_addr + 0x3E).write(temp1)};
		return -3;
	}
	// if nothing return 0;
	0
}

fn test_print(){
	let eth_dev = &mut ETH_DEV.lock();
    let rb_start_temp : u32 = unsafe{Port::new(eth_dev.base_addr + 0x30).read()};
    let capr_temp : u16 = unsafe{Port::new(eth_dev.base_addr + 0x38).read()};
    let cbr_temp : u16 = unsafe{Port::new(eth_dev.base_addr + 0x3A).read()};
    println!("RBSTART {:#x}",rb_start_temp);
    println!("CAPR {}",capr_temp as i16);
    println!("CBR {} and here is rx_buffer",cbr_temp);
    for i  in 0..cbr_temp as usize{
        print!("{:#x}\t",eth_dev.rx_buffer[i]);
    }
}

pub fn test_transmit(){
	const ETH_HEADER_SIZE : usize = 14;
    // const PACKET_SIZE : usize = 512;
    let src_mac = ETH_DEV.lock().get_mac_addr();
    let mut packet  = EthernetHeader::new(src_mac , [0xFF as u8 ; 6], 0x0800 as u16);
    // let mut packet : [u8; PACKET_SIZE] = [0;PACKET_SIZE];
    let packet_addr : *mut EthernetHeader = &mut packet;
    ETH_DEV.lock().transmit_packet(packet_addr as u32,ETH_HEADER_SIZE);

    // x86_64::instructions::interrupts::int43();
}
