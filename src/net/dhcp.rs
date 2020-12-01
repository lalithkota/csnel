use super::EthernetHeader;
use super::IPHeader;
use super::UDPHeader;
use super::DHCPHeader;

use crate::println;
use crate::eth_driver::EthDriver;
use crate::eth_driver::ETH_DEV;

pub fn dhcp_deal(eth_header : &EthernetHeader, ip_header : &IPHeader, udp_header : &UDPHeader, dhcp_header : &DHCPHeader, mapper : & crate::memory::OffsetPageTable<'static>){
	// println!("RECEIVED DHCP STUFF {:#?} {:#?}",dhcp_header.siaddr,dhcp_header.yiaddr);
	
	let options_starting = unsafe{&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8  as *const [u8;3])};
	unsafe{ETH_DEV.my_rx_ptr_offset += 3};
	
	if options_starting[0]==53 && options_starting[1]== 1 && options_starting[2]== 2{
		println!("DHCP OFFER REC {:?} {:?}",dhcp_header.siaddr,dhcp_header.yiaddr);
		
		const OPTIONS_SIZE_REQ : usize = (2+1)+(2+4)+(2+4)+1;
		
		let mut final_packet = (*eth_header,*ip_header,*udp_header,*dhcp_header,[0;OPTIONS_SIZE_REQ]);
		
		final_packet.0.src = eth_header.dest;
		final_packet.0.dest = [0xFF;6];
		
		final_packet.1.src = [0;4];
		final_packet.1.dest = [0xFF;4];
		final_packet.1.set_len(IPHeader::size_of() + UDPHeader::size_of() + DHCPHeader::size_of() + OPTIONS_SIZE_REQ as u16);
		final_packet.1.set_checksum(final_packet.1.calc_checksum(false));
		
		final_packet.3.op = 1;
		final_packet.3.ciaddr = dhcp_header.yiaddr;
		final_packet.3.yiaddr = [0;4];
		
		final_packet.4[00]=53;final_packet.4[01]=01;final_packet.4[02]=03;
		final_packet.4[03]=50;final_packet.4[04]=04;
		final_packet.4[05]=dhcp_header.yiaddr[0];
		final_packet.4[06]=dhcp_header.yiaddr[1];
		final_packet.4[07]=dhcp_header.yiaddr[2];
		final_packet.4[08]=dhcp_header.yiaddr[3];
		final_packet.4[09]=54;final_packet.4[10]=04;
		final_packet.4[11]=dhcp_header.siaddr[0];
		final_packet.4[12]=dhcp_header.siaddr[1];
		final_packet.4[13]=dhcp_header.siaddr[2];
		final_packet.4[14]=dhcp_header.siaddr[3];
		final_packet.4[15]=0xFF;
		
		final_packet.2.src_port = udp_header.dest_port;
		final_packet.2.dest_port = udp_header.src_port;
		final_packet.2.set_len(UDPHeader::size_of() + DHCPHeader::size_of() + OPTIONS_SIZE_REQ as u16);
		final_packet.2.set_checksum(final_packet.2.calc_checksum(false,&final_packet.1,&final_packet.3 as *const DHCPHeader as usize, DHCPHeader::size_of() as usize + OPTIONS_SIZE_REQ));
		
		use x86_64::VirtAddr;
		use crate::memory::MapperAllSizes;
		let phys = mapper.translate_addr(VirtAddr::new(&final_packet as *const (EthernetHeader,IPHeader,UDPHeader,DHCPHeader,[u8;OPTIONS_SIZE_REQ]) as u64));
		unsafe{ETH_DEV.transmit_packet(phys.unwrap().as_u64() as u32, (EthernetHeader::size_of() +IPHeader::size_of() + UDPHeader::size_of() + DHCPHeader::size_of()) as usize + OPTIONS_SIZE_REQ)};
		super::interrupt_poller();
	}
	else if options_starting[0] == 53 && options_starting[1] == 1 && options_starting[2] == 5{
		println!("DHCP ACK REC");
		
		let other_opts = unsafe{&*(&ETH_DEV.rx_buffer[ETH_DEV.my_rx_ptr_offset as usize] as *const u8 as *const [u8;50])};
		let mut size : usize = 0;
		loop{
			if other_opts[size]==0xff { size+=1; break; }
			size += other_opts[size+1] as usize + 2;
		}
		unsafe{ETH_DEV.my_rx_ptr_offset += size as u16};
		
		let mut ind : usize = 0;
		loop{
			if other_opts[ind] == 1{
				ind += 2;
				*super::ip::SUBNET_MASK.lock()=[other_opts[ind],other_opts[ind+1],other_opts[ind+2],other_opts[ind+3]];
				ind += 4;
			}
			else if other_opts[ind] == 3{
				ind += 2;
				*super::ip::ROUTER_IP.lock()=[other_opts[ind],other_opts[ind+1],other_opts[ind+2],other_opts[ind+3]];
				ind += 4;
			}
			else if other_opts[ind] == 51{
				ind += 2;
				*super::ip::LEASE_TIME.lock()=[other_opts[ind],other_opts[ind+1],other_opts[ind+2],other_opts[ind+3]];
				ind += 4;
			}
			else if other_opts[ind] == 54{
				ind += 2;
				*super::ip::DHCP_SERVER.lock()=[other_opts[ind],other_opts[ind+1],other_opts[ind+2],other_opts[ind+3]];
				ind += 4;
			}
			else if other_opts[ind] == 0xFF {
				break;
			}
			else{
				ind += other_opts[ind+1] as usize + 2;
			}
		}
		*super::ip::MY_IP.lock()=dhcp_header.yiaddr;
		
		println!("CONFIRMED DHCP RESULTS\nMY_IP {:?}\nRT_IP {:?}\nSUBNET_MASK {:?}\nDHCP_SERVER_addr {:?}\nLEASE_TIME {:?}",*super::ip::MY_IP.lock(),*super::ip::ROUTER_IP.lock(),*super::ip::SUBNET_MASK.lock(),*super::ip::DHCP_SERVER.lock(),*super::ip::LEASE_TIME.lock());
	}
	else if options_starting[0] == 53 && options_starting[1] == 1 && options_starting[2] == 6{
		println!("DHCP NAK REC");
		crate::hlt_loop();
	}
	else{
		println!("Some other DHCP STUFF {:?}",options_starting);
	}
}

pub fn dhcp_discover(mapper : & crate::memory::OffsetPageTable<'static>){
	const OPTIONS_SIZE : usize = (3)+(2+4)+1;
	let my_mac = unsafe{ETH_DEV.get_mac_addr()};
	let mut final_packet = (
		EthernetHeader{dest : [0xFF;6], src : my_mac, eth_type : [0x8,0x0]},
		IPHeader { 
			ver_ihl : (4<<4) | 5,
			tos : 0x10,
			total_len : [0,0],
			id : [0,0],
			flags_frag : [0,0],
			ttl : 64,
			protocol : 0x11,
			checksum : [0,0],
			src : [0;4],
			dest : [0xFF;4],
		},
		UDPHeader{src_port : [0,0x44],dest_port : [0,0x43],len : [0,0], checksum : [0,0]},
		DHCPHeader{
			op : 1, htype : 1, hlen : 0x6, hops : 0,
			xid : [0x10,0x29,0x38,0x47],
			secs : [0u8;2],
			flags : [0u8;2],
			ciaddr : [0u8;4],
			yiaddr : [0u8;4],
			siaddr : [0u8;4],
			giaddr : [0u8;4],
			chaddr : [0u8;4*4],
			legacy : [0u8;192],
			cookie : [0x63,0x82,0x53,0x63],
		},
		[0u8;OPTIONS_SIZE],
	);
	
	final_packet.1.set_len(IPHeader::size_of() + UDPHeader::size_of() + DHCPHeader::size_of() + OPTIONS_SIZE as u16);
	final_packet.1.set_checksum(final_packet.1.calc_checksum(false));
	
	for i in 0..6{
		final_packet.3.chaddr[i] = my_mac[i]
	}
	
	final_packet.4[00]=53; final_packet.4[01]=1; final_packet.4[02]=1;
	final_packet.4[03]=55; final_packet.4[04]=4; final_packet.4[05]=1; final_packet.4[06]=3; final_packet.4[07]=15; final_packet.4[08]=6;
	final_packet.4[09]=0xFF;
	
	
	final_packet.2.set_len(UDPHeader::size_of() + DHCPHeader::size_of() + OPTIONS_SIZE as u16);
	final_packet.2.set_checksum(final_packet.2.calc_checksum(false,&final_packet.1,&final_packet.3 as *const DHCPHeader as usize, DHCPHeader::size_of() as usize + OPTIONS_SIZE));
	
	use x86_64::VirtAddr;
	use crate::memory::MapperAllSizes;
	let phys = mapper.translate_addr(VirtAddr::new(&final_packet as *const (EthernetHeader,IPHeader,UDPHeader,DHCPHeader,[u8;OPTIONS_SIZE]) as u64));
	unsafe{ETH_DEV.transmit_packet(phys.unwrap().as_u64() as u32, (EthernetHeader::size_of() +IPHeader::size_of() + UDPHeader::size_of() + DHCPHeader::size_of()) as usize + OPTIONS_SIZE)};
	super::interrupt_poller();
}