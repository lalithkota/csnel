pub trait EthDriver {
    fn turn_on(&self);
    fn enable_bus_master(&self);
    // fn get_irq_no_from_pci_and_load(&self);
    fn reset_dev(&self);
    fn get_base_address(&self) -> u32;
    fn load_driver(&mut self);
    fn get_mac_addr(&self) -> [u8 ; 6];
    fn print_mac_addr(&self);
    fn update_rx_buffer_ptr(&mut self);
    fn mask_tok_rok(&self);
    fn set_receive_buff_rules(&self);
    fn start_te_re(&self);
    // fn transmit_packet(&mut self,packet_addr : u32, packet_size : usize);
}
