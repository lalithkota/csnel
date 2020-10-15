pub mod base;
pub mod pci_device;
mod check_bus;
mod check_device;
mod check_function;
mod check_pci_list;
mod msi;

pub static mut PCI_LIST : [pci_device::PciDevice; 32] = [pci_device::PciDevice::new(); 32];

pub fn pci_init(){
    check_bus::check_bus(0);

    check_pci_list::check_pci_list();

    // msi::print_sta_com();
    // msi::enable_msi();
}
