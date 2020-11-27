// use crate::print;
use crate::println;
use x86_64::instructions::port::Port;
use super::eth_driver_trait::EthDriver;
use crate::get_pci_dev_from_index;

// static mut CURRENT_DESC : u8 = 0;
// static mut RX_BUFFER : [u8 ; RX_BUFF_SIZE] = [0 ; RX_BUFF_SIZE];
// pub unsafe fn get_rx_buffer_ptr_as_u32() -> u32 {
//     let ret : *mut [u8 ; RX_BUFF_SIZE] = &mut RX_BUFFER;
//     ret as u32
// }

pub const RX_BUFF_SIZE : usize = 8192+16+1500;
pub const TSD : [u16;4] = [0x10,0x14,0x18,0x1C];
pub const TSAD : [u16;4] = [0x20,0x24,0x28,0x2C];

pub const CAPR : u16 = 0x0038;
pub const CBR : u16 = 0x003A;

pub struct RTL8139EthDriver {
    pub pci_dev_no : usize,
    pub base_addr : u16,
    pub current_desc : usize,
    pub rb_start : u32,
    pub my_rx_ptr_offset : u16,
    pub rx_buffer : [u8 ; RX_BUFF_SIZE],
}

impl RTL8139EthDriver {
    pub const fn new(index : usize) -> RTL8139EthDriver {
        RTL8139EthDriver {
            pci_dev_no : index,
            base_addr : 0x0,
            current_desc : 0x0,
            rb_start : 0,
            my_rx_ptr_offset : 0,
            rx_buffer : [0 ; RX_BUFF_SIZE],
        }
        // ret.rb_start = ret.get_rx_buffer_ptr_as_u32();
    }
    pub fn get_rx_buffer_ptr_as_u64(&mut self,mapper : &crate::memory::OffsetPageTable<'static>) -> u64 {
        use x86_64::structures::paging::MapperAllSizes;
        use x86_64::VirtAddr;
        let address : u64 = &mut self.rx_buffer as *mut [u8 ; RX_BUFF_SIZE] as u64;
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("rtl_rx_buffer adrees as follows : {:?} -> {:?}", virt, phys);
        phys.unwrap().as_u64()
    }

    pub fn transmit_packet(&mut self, packet_addr : u32, packet_size : usize){
        // let mut packet : [u8; PACKET_SIZE]= [0x00; PACKET_SIZE];
        // let packet_addr : *mut [u8; PACKET_SIZE] = &mut packet;
        unsafe{
            // let current_desc_usize = CURRENT_DESC as usize;

            Port::new(self.base_addr + TSAD[self.current_desc]).write(packet_addr);

            let mut temp_tsd : u32;// = Port::new(self.base_addr + TSD[self.current_desc]).read();
            temp_tsd = ( packet_size as u32 ) & !((1<<13) as u32);
            Port::new(self.base_addr + TSD[self.current_desc]).write(temp_tsd);

            loop {
                temp_tsd = Port::new(self.base_addr + TSD[self.current_desc]).read();
                if temp_tsd & 1<<14 != 0{
                    println!("TUN");
                }
                // if temp_tsd & 1<<13 != 0{
                //     println!("OWN set");
                // }
                if temp_tsd & 1<<15 != 0{
                    // println!("TOK");
                    break;
                }

            }
            // println!("Out of loop");

            self.current_desc = (self.current_desc + 1)%4;
        }
    }
}

impl EthDriver for RTL8139EthDriver {
    fn load_driver(&mut self){
        self.base_addr = self.get_base_address() as u16;
        println!("base_addr : {:#x}", self.base_addr);

        self.print_mac_addr();

        // self.get_irq_no_from_pci_and_load();

        self.turn_on();
        self.enable_bus_master();

        self.reset_dev();

        self.update_rx_buffer_ptr();
        self.mask_tok_rok();
        self.set_receive_buff_rules();
        self.start_te_re();

        // self.transmit_packet();
    }

    fn get_base_address(&self) -> u32{
        let pci_dev = get_pci_dev_from_index!(self.pci_dev_no);
        let mut bar0 = pci_dev.get_bar_with_no(0,0);
        if bar0 & 0x1 == 0 {
            bar0=0x0;
        }
        bar0 & 0xFFFFFFFC
    }

    fn enable_bus_master(&self){
        let pci_dev = get_pci_dev_from_index!(self.pci_dev_no);
        println!("VID-DID:{:#x}-{:#x} ; Command reg : {:#x}",pci_dev.get_vendor_id(),pci_dev.get_device_id(),pci_dev.get_command_reg(0));
        pci_dev.enable_bus_master(0);
        println!("Command reg : {:#x}",pci_dev.get_command_reg(0));
    }

    // fn get_irq_no_from_pci_and_load(&self){
    //     let pci_dev = get_pci_dev_from_index!(self.pci_dev_no);
    //     let irq_line = (pci_dev.get_irq_pin_line(0) & 0xFF) as usize;
    //     (interrupts::IDT.lock())[32 + irq_line].set_handler_fn(rtl8139_irq_handler);
    // }

    fn get_mac_addr(&self) -> [u8;6]{
        let mac_addr_0_4 : u32 = unsafe { Port::new(self.base_addr).read()};
        let mac_addr_5_8 : u32 = unsafe { Port::new(self.base_addr+4).read()};
        let ret : [u8 ; 6] = [(mac_addr_5_8 >> 8) as u8, (mac_addr_5_8 >> 0) as u8, (mac_addr_0_4 >> 24) as u8, (mac_addr_0_4 >> 16) as u8, (mac_addr_0_4 >> 8) as u8, (mac_addr_0_4 >> 0) as u8];
        ret
    }
    fn print_mac_addr(&self){
        let ret : [u8 ; 6] = self.get_mac_addr();
        println!("MAC addr : {:#X}-{:#X}-{:#X}-{:#X}-{:#X}-{:#X}",ret[5],ret[4],ret[3],ret[2],ret[1],ret[0]);
    }

    fn turn_on(&self){
        unsafe{ Port::new(self.base_addr + 0x52).write(0x0 as u8) };
    }

    fn reset_dev(&self){
        unsafe{ Port::new(self.base_addr + 0x37).write(0x10 as u8);}
        let mut tmp_byte : u8 = unsafe{Port::new(self.base_addr + 0x37).read()};
        while ( tmp_byte & 0x10 ) != 0 {
            tmp_byte = unsafe{ Port::new(self.base_addr + 0x37).read()};
        }
        println!("Finished , Received RST");
    }

    fn update_rx_buffer_ptr(&mut self) -> bool{

        if self.rb_start==0{
            return false;
        }
        unsafe{Port::new(self.base_addr + 0x30).write(self.rb_start);}
        true
    }

    fn mask_tok_rok(&self){
        unsafe{Port::new(self.base_addr + 0x3C).write(0xFFFF as u16);}
    }

    fn set_receive_buff_rules(&self){
        unsafe{Port::new(self.base_addr + 0x44).write((0b1111 | (0b1 << 7)) as u32);}
    }

    fn start_te_re(&self){
        unsafe{Port::new(self.base_addr + 0x37).write(0x0C as u8);}
    }
}
