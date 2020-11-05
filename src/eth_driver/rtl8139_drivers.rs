// use crate::print;
use crate::println;
use x86_64::instructions::port::Port;
use super::eth_driver_trait::EthDriver;
use crate::get_pci_dev_from_index;

const RX_BUFF_SIZE : usize = 8192+16+1500;

static mut CURRENT_DESC : u8 = 0;

const TSD : [u16;4] = [0x10,0x14,0x18,0x1C];
const TSAD : [u16;4] = [0x20,0x24,0x28,0x2C];

static mut RX_BUFFER : [u8 ; RX_BUFF_SIZE] = [0 ; RX_BUFF_SIZE];

pub unsafe fn get_rx_buffer_ptr_as_u32() -> u32 {
    let ret : *mut [u8 ; RX_BUFF_SIZE] = &mut RX_BUFFER;
    ret as u32
}

pub struct RTL8139EthDriver {
    pub pci_dev_no : usize,
    pub base_addr : u16,
}

impl RTL8139EthDriver {
    pub fn new(index : usize) -> RTL8139EthDriver {
        RTL8139EthDriver {
            pci_dev_no : index,
            base_addr : 0x0,
        }
    }
}

impl EthDriver for RTL8139EthDriver {
    fn load_driver(&mut self){
        self.enable_bus_master();

        self.base_addr = self.get_base_address() as u16;
        println!("base_addr : {:#x}",self.base_addr);

        self.print_mac_addr();

        self.turn_on();

        self.reset_dev();

        self.update_buffer_ptr();
        self.mask_tok_rok();
        self.start_te_re();

        self.transmit_packet();
    }

    fn transmit_packet(&self){
        const PACKET_SIZE : usize = 4;
        let mut packet : [u8; PACKET_SIZE]= [0x00; PACKET_SIZE];
        let packet_addr : *mut [u8; PACKET_SIZE] = &mut packet;
        unsafe{
            let current_desc_usize = CURRENT_DESC as usize;

            Port::new(self.base_addr + TSAD[current_desc_usize]).write(packet_addr as u32);

            let mut temp_tsd : u32 = Port::new(self.base_addr + TSD[current_desc_usize]).read();
            temp_tsd = ( temp_tsd | PACKET_SIZE as u32 ) & !((1<<13) as u32);
            Port::new(self.base_addr + TSD[current_desc_usize]).write(temp_tsd);

            loop {
                temp_tsd = Port::new(self.base_addr + TSD[current_desc_usize]).read();
                if temp_tsd & 1<<14 != 0{
                    println!("TUN");
                    break;
                }
                if temp_tsd & 1<<15 != 0{
                    println!("TOK");
                    break;
                }
                // if temp_tsd & 1<<13 != 0{
                //     print!("OWN set");
                // }
            }
            println!("Out of loop");

            CURRENT_DESC = (CURRENT_DESC + 1)%4;
        }
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

    fn print_mac_addr(&self){
        let mac_addr_0_4 : u32 = unsafe { Port::new(self.base_addr).read()};
        let mac_addr_5_8 : u32 = unsafe { Port::new(self.base_addr+4).read()};
        println!("MAC addr : {:#X}-{:#X}-{:#X}-{:#X}-{:#X}-{:#X}",(mac_addr_0_4 >> 0) as u8, (mac_addr_0_4 >> 8) as u8 ,(mac_addr_0_4 >> 16) as u8 , (mac_addr_0_4 >> 24) as u8, (mac_addr_5_8 >> 0) as u8 , (mac_addr_5_8 >> 8) as u8);
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

    fn update_buffer_ptr(&self){
        unsafe{Port::new(self.base_addr + 0x30).write(get_rx_buffer_ptr_as_u32());}
    }

    fn mask_tok_rok(&self){
        unsafe{Port::new(self.base_addr + 0x3C).write(0x5 as u16);}
    }

    fn set_receive_buff_rules(&self){
        unsafe{Port::new(self.base_addr + 0x44).write((0b10001111) as u32);}
    }

    fn start_te_re(&self){
        unsafe{Port::new(self.base_addr + 0x37).write(0x0C as u8);}
    }
}
