pub mod base;
pub mod pci_device;
mod check_bus;
mod check_device;
mod check_function;
mod check_pci_list;
mod msi;

use lazy_static::lazy_static;
use spin;

pub struct PciList{
    pub list : [pci_device::PciDevice; 32],
}
impl PciList{
    pub fn load(&self){
        // do nothing
        // dummy func
    }
}

lazy_static! {
    pub static ref PCI_LIST: spin::Mutex<PciList> = spin::Mutex::new({
        let mut ret = PciList{ list : [pci_device::PciDevice::new() ; 32] };
        let count = check_bus::check_bus_and_get_list(0,&mut ret.list,0); // in future loop this ... also use count
        ret
    });
}


pub fn pci_init(){
    // check_bus::check_bus(0);
    PCI_LIST.lock().load();
    check_pci_list::check_pci_list();

    // msi::_print_sta_com();
    // msi::_enable_msi();
}

#[macro_export]
macro_rules! get_pci_dev_from_index {
    ($index : expr) => {
        crate::pci::PCI_LIST.lock().list[$index]
    };
}
