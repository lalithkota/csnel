use super::EthernetHeader;
use super::IPHeader;
use super::UDPHeader;
use super::DHCPHeader;

use crate::println;
use crate::eth_driver::ETH_DEV;

pub fn udp_deal(eth_header : &EthernetHeader, ip_header : &IPHeader, udp_header : &UDPHeader, mapper : & crate::memory::OffsetPageTable<'static>){
	if compare_ports(udp_header.src_port, 67) && compare_ports(udp_header.dest_port, 68) {
		let dhcp_header = unsafe{&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8  as *const DHCPHeader)};
		unsafe{ETH_DEV.my_rx_ptr_offset += DHCPHeader::size_of()};
		
		super::dhcp::dhcp_deal(eth_header,ip_header,udp_header,dhcp_header,mapper);
	}
	else{
		println!("Other UDP packets rec");
	}
}

pub fn compare_ports(a : [u8;2], b : u16) -> bool{
	(((a[0] as u16) <<8)|(a[1] as u16)) == b
}

impl UDPHeader{
	pub fn calc_checksum(&self, check : bool, ip_header : &IPHeader, payload_ptr : usize, payload_size : usize) -> u16{
		let mut sum : u32 = 0;

		//psuedo ip header checksum calc
		sum += ((ip_header.src[0] as u32) <<8 )| (ip_header.src[1] as u32);
		sum += ((ip_header.src[2] as u32) <<8 )| (ip_header.src[3] as u32);

		sum += ((ip_header.dest[0] as u32) <<8 )| (ip_header.dest[1] as u32);
		sum += ((ip_header.dest[2] as u32) <<8 )| (ip_header.dest[3] as u32);

		sum += ip_header.protocol as u32;

		sum += UDPHeader::size_of() as u32 + payload_size as u32;
		// end of pseudo ip header checksum

		sum += ((self.src_port[0] as u32) <<8 )| (self.src_port[1] as u32);
		sum += ((self.dest_port[0] as u32) <<8 )| (self.dest_port[1] as u32);

		sum += ((self.len[0] as u32) <<8 )| (self.len[1] as u32);
		if check {
			sum += ((self.checksum[0] as u32) <<8 )| (self.checksum[1] as u32);
		}

		for i in 0..payload_size/2{
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