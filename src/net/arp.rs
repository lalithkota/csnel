use crate::println;

use crate::eth_driver::EthDriver;
use crate::eth_driver::ETH_DEV;

use crate::memory::OffsetPageTable;
use crate::memory::MapperAllSizes;

use super::EthernetHeader;
use super::ARPHeader;

pub fn arp_deal(arp_req : &ARPHeader, eth_header : &EthernetHeader, mapper : &OffsetPageTable<'static>){
	println!("arp deal");
	let mut final_packet = (*eth_header,*arp_req);
	final_packet.0.src = unsafe{ETH_DEV.get_mac_addr()};
	final_packet.0.dest = eth_header.src;

	final_packet.1.set_oper(2 as u16);
	final_packet.1.sender_hw_addr = final_packet.0.src;
	final_packet.1.sender_p_addr = arp_req.receiver_p_addr;
	final_packet.1.receiver_hw_addr = final_packet.0.dest;
	final_packet.1.receiver_p_addr = arp_req.sender_p_addr;
	//arp_req.pretty_print();
	//final_packet.1.pretty_print();
	//crate::hlt_loop();
	use x86_64::VirtAddr;
	let phys = mapper.translate_addr(VirtAddr::new(&final_packet as *const (EthernetHeader,ARPHeader) as u64));
	unsafe{ETH_DEV.transmit_packet(phys.unwrap().as_u64() as u32, (EthernetHeader::size_of() + ARPHeader::size_of()) as usize)};
	super::interrupt_poller();
}
