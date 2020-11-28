use crate::println;

#[derive(Copy,Clone)]
#[repr(C)]
pub struct TCPHeader {
    pub src_port: [u8;2],
	pub dest_port: [u8;2],
	pub seq_no : [u8;4],
	pub ack_no : [u8;4],
    pub data_offset : u8,
	pub flags : u8,
	pub win_size : [u8;2],
	pub checksum : [u8;2],
	pub urg_ptr : [u8;2],
}

impl TCPHeader{
    pub fn pretty_print(&self){
        println!("TCP HEADER");
        println!("source  port{}",((self.src_port[0] as u16) << 8)| self.src_port[1] as u16 );
        println!("dest  port{}",((self.dest_port[0] as u16) << 8)| self.dest_port[1] as u16 );
        println!("seq no{:#x}",((self.seq_no[0] as u32) << 24)| ((self.seq_no[1] as u32) << 16) | ((self.seq_no[2] as u32) << 8) | (self.seq_no[3] as u32));
        println!("ack no{:#x}",((self.ack_no[0] as u32) << 24)| ((self.ack_no[1] as u32) << 16) | ((self.ack_no[2] as u32) << 8) | (self.ack_no[3] as u32));

    }

    pub fn size_of() -> u16{
        (2+2+4+4+1+1+2+2+2) as u16
    }

	pub fn get_seq_no(&self) -> u32{
		((self.seq_no[0] as u32) << 24) | ((self.seq_no[1] as u32) << 16) | ((self.seq_no[2] as u32) << 8) | ((self.seq_no[3] as u32) << 0)
	}
	pub fn get_ack_no(&self) -> u32{
		((self.ack_no[0] as u32) << 24) | ((self.ack_no[1] as u32) << 16) | ((self.ack_no[2] as u32) << 8) | ((self.ack_no[3] as u32) << 0)
	}

    pub fn get_data_off(&self) -> u8{
        self.data_offset >> 4

    }


	pub fn set_seq_no(&mut self,seq : u32){
		self.seq_no[0] = (seq>>24) as u8;
		self.seq_no[1] = (seq>>16) as u8;
		self.seq_no[2] = (seq>>8) as u8;
		self.seq_no[3] = (seq>>0) as u8;
	}
	pub fn set_ack_no(&mut self, ack : u32){
		self.ack_no[0] = (ack>>24) as u8;
		self.ack_no[1] = (ack>>16) as u8;
		self.ack_no[2] = (ack>>8) as u8;
		self.ack_no[3] = (ack>>0) as u8;
	}
	pub fn set_win_size(&mut self, win : u16){
		self.win_size[0] = ((win>>8) & 0xFF) as u8;
		self.win_size[1] = (win & 0xFF) as u8;
	}
	pub fn set_checksum(&mut self, chec : u16){
		self.checksum[0] = ((chec>>8) & 0xFF) as u8;
		self.checksum[1] = (chec & 0xFF) as u8;
	}

	pub fn is_fin(&self) -> bool{
		self.flags & (1<<0) != 0
	}
	pub fn is_syn(&self) -> bool{
		self.flags & (1<<1) != 0
	}
	pub fn is_reset(&self) -> bool{
		self.flags & (1<<2) != 0
	}
	pub fn is_push(&self) -> bool{
		self.flags & (1<<3) != 0
	}
	pub fn is_ack(&self) -> bool{
		self.flags & (1<<4) != 0
	}

	pub fn set_fin(&mut self, flag : bool){
		if flag {
			self.flags |= 1<<0;
		}
		else{
			self.flags &= !(1<<0);
		}
	}
	pub fn set_syn(&mut self, flag : bool){
		if flag {
			self.flags |= 1<<1;
		}
		else{
			self.flags &= !(1<<1);
		}
	}
	pub fn set_reset(&mut self, flag : bool){
		if flag {
			self.flags |= 1<<2;
		}
		else{
			self.flags &= !(1<<2);
		}
	}
	pub fn set_push(&mut self, flag : bool){
		if flag {
			self.flags |= 1<<3;
		}
		else{
			self.flags &= !(1<<3);
		}
	}
	pub fn set_ack(&mut self, flag : bool){
		if flag {
			self.flags |= 1<<4;
		}
		else{
			self.flags &= !(1<<4);
		}
	}

}

#[derive(Copy,Clone)]
#[repr(C)]
pub struct IPHeader {
    pub ver_ihl: u8,
    pub tos: u8,
	pub total_len : [u8;2],
	pub id : [u8;2],
	pub flags_frag : [u8;2],
	pub ttl : u8,
	pub protocol : u8,
	pub checksum : [u8;2],
	pub src : [u8;4],
	pub dest : [u8;4],
}

impl IPHeader{
    pub fn pretty_print(&self){
    }

    pub fn size_of() -> u16{
        (1+1+2+2+2+1+1+2+4+4) as u16
    }
	pub fn set_len(&mut self, length : u16){
		self.total_len[0] = ((length>>8) & 0xFF) as u8;
		self.total_len[1] = (length & 0xFF) as u8;
	}
	pub fn set_checksum(&mut self, chec : u16){
		self.checksum[0] = ((chec>>8) & 0xFF) as u8;
		self.checksum[1] = (chec & 0xFF) as u8;
	}
	pub fn is_tcp(&self) -> bool{
		self.protocol == 0x06
	}

    pub fn get_len(&self) -> u16{
        ((self.total_len[0] as u16) << 8) | (self.total_len[1] as u16)
        }
}

#[derive(Copy,Clone)]
#[repr(C)]
pub struct ARPHeader {
    pub htype: [u8; 2],
    pub ptype: [u8; 2],
	pub hw_len : u8,
	pub p_len : u8,
	pub oper : [u8;2],
	pub sender_hw_addr : [u8;6],
	pub sender_p_addr : [u8;4],
	pub receiver_hw_addr : [u8;6],
	pub receiver_p_addr : [u8;4],
}

impl ARPHeader{
    pub fn pretty_print(&self){
        println!("ARPHeader");
        println!("\t htype {:#x}",((self.htype[0] as u16)<< 8) |(self.htype[1] as u16));
        println!("\t ptype {:#x}",((self.ptype[0] as u16)<< 8) |(self.ptype[1] as u16));
        println!("\t hw_len {:#x}",self.hw_len);
        println!("\t p_len {:#x}",self.p_len);
        println!("\t oper {:#x}",((self.oper[0] as u16)<< 8) |(self.oper[1] as u16));
        println!("\t sender_hw_addr {:#x} {:#x} {:#x} {:#x} {:#x} {:#x}",self.sender_hw_addr[0],self.sender_hw_addr[1],self.sender_hw_addr[2],self.sender_hw_addr[3],self.sender_hw_addr[4],self.sender_hw_addr[5]);
        println!("\t sender_p_addr {:#x} {:#x} {:#x} {:#x}",self.sender_p_addr[0],self.sender_p_addr[1],self.sender_p_addr[2],self.sender_p_addr[3]);
        println!("\t receiver_hw_addr {:#x} {:#x} {:#x} {:#x} {:#x} {:#x}",self.receiver_hw_addr[0],self.receiver_hw_addr[1],self.receiver_hw_addr[2],self.receiver_hw_addr[3],self.receiver_hw_addr[4],self.receiver_hw_addr[5]);
        println!("\t receiver_p_addr {:#x} {:#x} {:#x} {:#x}",self.receiver_p_addr[0],self.receiver_p_addr[1],self.receiver_p_addr[2],self.receiver_p_addr[3]);
    }

    pub fn size_of() -> u16{
        (2+2+1+1+2+6+4+6+4) as u16
    }
	pub fn set_oper(&mut self, op : u16){
		self.oper[0] = ((op>>8) & 0xFF) as u8;
		self.oper[1] = (op & 0xFF) as u8;
	}
}

#[derive(Copy,Clone)]
#[repr(C)]
pub struct EthernetHeader {
    pub dest: [u8; 6],
    pub src: [u8; 6],
    pub eth_type: [u8;2],
}

impl EthernetHeader {
    pub fn new(src: [u8; 6], dest: [u8; 6], eth_type: u16) -> EthernetHeader {
        EthernetHeader {
            dest: dest,
            src: src,
            eth_type: [((eth_type >> 8)& 0xFF) as u8,(eth_type & 0xFF) as u8 ],
        }
    }
    pub fn pretty_print(&self){
    }

    pub fn size_of() -> u16{
        (6+6+2) as u16
    }
	pub fn get_type(&self) -> u16{
		((self.eth_type[0] as u16) << 8) | (self.eth_type[1] as u16)
	}
}
