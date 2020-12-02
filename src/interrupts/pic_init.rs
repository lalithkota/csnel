use pic8259_simple::ChainedPics;
use spin;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(32,32+8) });
