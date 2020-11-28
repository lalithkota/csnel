use super::IPHeader;

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
