use crate::println;
use crate::eth_driver::ETH_DEV;

use super::EthernetHeader;
use super::IPHeader;
use super::TCPHeader;
use super::UDPHeader;

pub static MY_IP : spin::Mutex<[u8;4]> = spin::Mutex::new([0;4]);
pub static ROUTER_IP : spin::Mutex<[u8;4]> = spin::Mutex::new([0;4]);
pub static DHCP_SERVER : spin::Mutex<[u8;4]> = spin::Mutex::new([0;4]);
pub static SUBNET_MASK : spin::Mutex<[u8;4]> = spin::Mutex::new([0;4]);
pub static LEASE_TIME : spin::Mutex<[u8;4]> = spin::Mutex::new([0;4]);


pub fn ip_deal(ip_header: &IPHeader, eth_header : &EthernetHeader, mapper : &crate::memory::OffsetPageTable<'static>){
	if compare_ip(ip_header.dest, [0xff;4]) {
		// might be dhcp
	}
	else if !compare_ip(ip_header.dest, *MY_IP.lock()){
		println!("Some other ip");
		return;
	}
	
	if ip_header.is_udp() {
		let udp_header = unsafe{&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8  as *const UDPHeader)};
		unsafe{ETH_DEV.my_rx_ptr_offset += UDPHeader::size_of()};
		
		super::udp::udp_deal(eth_header,ip_header,udp_header,mapper);
	}
	else if ip_header.is_tcp() {
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
				super::tcp::tcp_opt_deal(eth_header, ip_header, tcp_header, options, option_size, mapper);
			}
			else if tcp_header.get_data_off() == 5 {
				let option_size = 0;
				let options : &[u8;40] = &[0 as u8;40];
				super::tcp::tcp_opt_deal(eth_header, ip_header, tcp_header, options, option_size, mapper);
			}
		}
		else if (!tcp_header.is_syn()) & tcp_header.is_ack(){
			super::tcp::tcp_deal(eth_header, ip_header, tcp_header, mapper);
		}
	}
	else{
		println!("Some other ip packet received");
	}
}

impl IPHeader{
	pub fn calc_checksum(&self, check : bool) -> u16{
		let mut sum : u32 = 0;
		sum += ((self.ver_ihl as u32)<<8) | ((self.tos as u32));
		sum += ((self.total_len[0] as u32)<<8) | ((self.total_len[1] as u32));
		sum += ((self.id[0] as u32)<<8) | ((self.id[1] as u32));
		sum += ((self.flags_frag[0] as u32)<<8) | ((self.flags_frag[1] as u32));
		sum += ((self.ttl as u32)<<8) | ((self.protocol as u32));
		if check {
			sum += ((self.checksum[0] as u32)<<8) | ((self.checksum[1] as u32));
		}
		sum += ((self.src[0] as u32)<<8) | ((self.src[1] as u32));
		sum += ((self.src[2] as u32)<<8) | ((self.src[3] as u32));

		sum += ((self.dest[0] as u32)<<8) | ((self.dest[1] as u32));
		sum += ((self.dest[2] as u32)<<8) | ((self.dest[3] as u32));
		if (sum&0xFFFF0000) > 0{
			sum = (sum&0x0000FFFF) + ((sum>>16) & 0x0000FFFF);
			crate::println!("in if");
		}
		// this is mandatory to do twice
		if (sum&0xFFFF0000) > 0{
			sum = (sum&0x0000FFFF) + ((sum>>16) & 0x0000FFFF);
		}

		!(sum) as u16
	}
}

pub fn compare_ip(a : [u8;4], b : [u8;4]) -> bool{
	a[0] == b[0] &&
	a[1] == b[1] &&
	a[2] == b[2] &&
	a[3] == b[3]
}
