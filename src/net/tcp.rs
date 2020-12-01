use super::EthernetHeader;
use super::IPHeader;
use super::TCPHeader;
use crate::println;
use crate::eth_driver::ETH_DEV;

// const TEMP_HTTP_RESPONSE :&str = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\n<html>CSNEL IS GOOd & NICE</html>\r\n\n";
use super::temp_http_response::TEMP_HTTP_RESPONSE;

pub static TCP_ACCEPT_PORT : spin::Mutex<u16> = spin::Mutex::new(80);

pub fn tcp_deal(eth_header : &EthernetHeader, ip_header : &IPHeader, tcp_header : &TCPHeader, mapper : & crate::memory::OffsetPageTable<'static>){
	println!("regular tcp deal");
	
	if !compare_ports(tcp_header.dest_port, *TCP_ACCEPT_PORT.lock()){
		println!("some other port {} {}",((tcp_header.dest_port[0] as u16) <<8)|(tcp_header.dest_port[1] as u16),*TCP_ACCEPT_PORT.lock());
		return;
	}
	
	let pack_size = ip_header.get_len()-(IPHeader::size_of()+TCPHeader::size_of());
	println!("pack size {} ",pack_size);
	if pack_size> 0{
		let tcp_packet = unsafe {&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8  as *const [u8;800])};
		unsafe{ETH_DEV.my_rx_ptr_offset+= pack_size};
		
		const HTTP_REQ_CHECK : &str = "GET /";
		let pack_header_as_string = core::str::from_utf8(&tcp_packet[0..HTTP_REQ_CHECK.len()]).unwrap();
		
		if pack_header_as_string == HTTP_REQ_CHECK{
			
			// using str.len() is not right.. :TODO
			println!("HTTP REQUEST REC!");
			let packet_as_utf8 = core::str::from_utf8(&tcp_packet[0..pack_size as usize]).unwrap();
			// println!("{}",packet_as_utf8);
			
			let mut final_packet = (*eth_header,*ip_header, *tcp_header, [0u8;800]);
			for i in 0..TEMP_HTTP_RESPONSE.len(){
				final_packet.3[i] = TEMP_HTTP_RESPONSE.as_bytes()[i];
			}
			
			final_packet.0.src = eth_header.dest;
			final_packet.0.dest = eth_header.src;
			
			final_packet.1.src = ip_header.dest;
			final_packet.1.dest = ip_header.src;
			final_packet.1.set_len(IPHeader::size_of() + TCPHeader::size_of() + TEMP_HTTP_RESPONSE.len() as u16);
			final_packet.1.set_checksum(final_packet.1.calc_checksum(false));
			
			final_packet.2.src_port = tcp_header.dest_port;
			final_packet.2.dest_port = tcp_header.src_port;
			final_packet.2.set_seq_no(tcp_header.get_seq_no() + 89809);
			final_packet.2.set_ack_no(tcp_header.get_seq_no()+1);
			final_packet.2.set_ack(true);
			final_packet.2.set_checksum(final_packet.2.calc_checksum(false,&final_packet.1, &final_packet.3 as *const [u8] as *const u8 as usize, TEMP_HTTP_RESPONSE.len()));
			
			use x86_64::VirtAddr;
			use crate::memory::MapperAllSizes;
			use crate::eth_driver::eth_driver_trait::EthDriver;
			let phys = mapper.translate_addr(VirtAddr::new(&final_packet as *const (EthernetHeader,IPHeader,TCPHeader,[u8;800]) as u64));
			unsafe{ETH_DEV.transmit_packet(phys.unwrap().as_u64() as u32, (EthernetHeader::size_of() +IPHeader::size_of() + TCPHeader::size_of()) as usize + TEMP_HTTP_RESPONSE.len())};
			super::interrupt_poller();
		}
		
		crate::hlt_loop();
	}
}

pub fn tcp_opt_deal(eth_header : &EthernetHeader, ip_header : &IPHeader, tcp_header : &TCPHeader, options : &[u8;40], option_size : u16, mapper : &crate::memory::OffsetPageTable<'static>){
	if tcp_header.is_syn() {
		let mut final_packet = (*eth_header,*ip_header,*tcp_header,*options);

		final_packet.0.src = eth_header.dest;
		final_packet.0.dest = eth_header.src;

		final_packet.1.src = ip_header.dest;
		final_packet.1.dest = ip_header.src;
		final_packet.1.set_len(IPHeader::size_of() + TCPHeader::size_of() + option_size);
		final_packet.1.set_checksum(final_packet.1.calc_checksum(false));

		final_packet.2.src_port = tcp_header.dest_port;
		final_packet.2.dest_port = tcp_header.src_port;
		final_packet.2.set_seq_no(tcp_header.get_seq_no()+ 89809);
		final_packet.2.set_ack_no(tcp_header.get_seq_no()+1);
		final_packet.2.set_syn(true);
		final_packet.2.set_ack(true);
		final_packet.2.set_checksum(final_packet.2.calc_checksum(false,&final_packet.1,options as *const [u8;40] as usize, option_size as usize));

		use x86_64::VirtAddr;
		use crate::memory::MapperAllSizes;
		use crate::eth_driver::eth_driver_trait::EthDriver;
		let phys = mapper.translate_addr(VirtAddr::new(&final_packet as *const (EthernetHeader,IPHeader,TCPHeader,[u8;40]) as u64));
		unsafe{ETH_DEV.transmit_packet(phys.unwrap().as_u64() as u32, (EthernetHeader::size_of() +IPHeader::size_of() + TCPHeader::size_of() + option_size) as usize)};
		super::interrupt_poller();


		//println!("ip_checksum {:#x} {:#x}",ip_header.calc_checksum(false),((ip_header.checksum[0] as u16) << 8) |ip_header.checksum[1] as u16 );
		//println!("tcp_checksum {:#x} {:#x}",tcp_header.calc_checksum(false, ip_header,options as *const [u8;40] as usize, option_size as usize),((tcp_header.checksum[0] as u16) << 8) |tcp_header.checksum[1] as u16);
	}
}
pub fn compare_ports(a : [u8;2], b : u16) -> bool{
	(((a[0] as u16) <<8)|(a[1] as u16)) == b
}

impl TCPHeader{
	pub fn calc_checksum(&self, check : bool, ip_header : &IPHeader, payload_ptr : usize, payload_size : usize) -> u16{
		let mut sum : u32 = 0;

		//psuedo ip header checksum calc
		sum += ((ip_header.src[0] as u32) <<8 )| (ip_header.src[1] as u32);
		sum += ((ip_header.src[2] as u32) <<8 )| (ip_header.src[3] as u32);

		sum += ((ip_header.dest[0] as u32) <<8 )| (ip_header.dest[1] as u32);
		sum += ((ip_header.dest[2] as u32) <<8 )| (ip_header.dest[3] as u32);

		sum += ip_header.protocol as u32;

		sum += TCPHeader::size_of() as u32 + payload_size as u32;
		// end of pseudo ip header checksum

		sum += ((self.src_port[0] as u32) <<8 )| (self.src_port[1] as u32);
		sum += ((self.dest_port[0] as u32) <<8 )| (self.dest_port[1] as u32);

		sum += ((self.seq_no[0] as u32) <<8 )| (self.seq_no[1] as u32);
		sum += ((self.seq_no[2] as u32) <<8 )| (self.seq_no[3] as u32);

		sum += ((self.ack_no[0] as u32) <<8 )| (self.ack_no[1] as u32);
		sum += ((self.ack_no[2] as u32) <<8 )| (self.ack_no[3] as u32);

		sum += ((self.data_offset as u32) <<8 )| (self.flags as u32);

		sum += ((self.win_size[0] as u32) <<8 )| (self.win_size[1] as u32);
		if check {
			sum += ((self.checksum[0] as u32) <<8 )| (self.checksum[1] as u32);
		}
		sum += ((self.urg_ptr[0] as u32) <<8 )| (self.urg_ptr[1] as u32);

		for i in 0..payload_size/2{
			//println!("value of index{} is {:#x}",i,(((unsafe{*((payload_ptr + 2*i) as *const u8)}) as u32) << 16) |  ((unsafe{*((payload_ptr + 2*i+1) as *const u8)}) as u32));
			sum += (((unsafe{*((payload_ptr + 2*i) as *const u8)}) as u32) << 8) |  ((unsafe{*((payload_ptr + 2*i+1) as *const u8)}) as u32);
		}

		if (sum&0xFFFF0000) > 0{
			sum = (sum&0x0000FFFF) + ((sum>>16) & 0x0000FFFF);
		}
		// this is mandatory to do twice
		if (sum&0xFFFF0000) > 0{
			sum = (sum&0x0000FFFF) + ((sum>>16) & 0x0000FFFF);
		}

		!(sum) as u16

	}
}
