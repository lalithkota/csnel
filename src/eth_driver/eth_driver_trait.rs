pub trait EthDriver {
    fn turn_on(&self);
    fn enable_bus_master(&self);
    fn reset_dev(&self);
    fn get_base_address(&self) -> u32;
    fn load_driver(&mut self);
    fn print_mac_addr(&self);
    fn update_buffer_ptr(&self);
    fn mask_tok_rok(&self);
    fn set_receive_buff_rules(&self);
    fn start_te_re(&self);
    fn transmit_packet(&self);
}
