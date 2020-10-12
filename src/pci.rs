mod base;
mod check_bus;
mod check_device;
mod check_function;

pub fn pci_init(){
    check_bus::check_bus(0);
}
