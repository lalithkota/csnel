pub mod gdt;
pub mod pic_init;

pub use x86_64::structures::idt::InterruptDescriptorTable;
pub use x86_64::structures::idt::InterruptStackFrame;
use crate::println;
use crate::print;
use lazy_static::lazy_static;
use spin::Mutex;
use pic_init::PICS;

lazy_static! {
	pub static ref IRQ_HANDLERS : Mutex<[fn(stack_frame : &mut InterruptStackFrame);16]>= Mutex::new([default_irq_hand;16]);
	
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // CPU Exceptions
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);}
		idt.page_fault.set_handler_fn(page_fault_handler);
		
        // Hardware Interrupts
        idt[32].set_handler_fn(irq_0_handler);
        idt[32+1].set_handler_fn(irq_1_handler);
        idt[32+11].set_handler_fn(irq_11_handler);

        idt
    };
}

pub fn init_interrupts() -> bool{
    // for gdt
	gdt::init();
	
	IRQ_HANDLERS.lock()[0]=timer_interrupt_handler;
	IRQ_HANDLERS.lock()[1]=keyboard_interrupt_handler;
    
	// for idt
	IDT.load();
    
	// for pic
	unsafe { pic_init::PICS.lock().initialize()};
    
	// enable
	x86_64::instructions::interrupts::enable();
	
	true
}

//CPU Exception Handlers
extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn page_fault_handler(stack_frame: &mut InterruptStackFrame, error_code : x86_64::structures::idt::PageFaultErrorCode)
{
    panic!("EXCEPTION: PAGE FAULT\n{:#?}\nError Code: {:#?}", stack_frame,error_code);
}

//Hardware Interrupt Handlers
pub extern "x86-interrupt" fn irq_0_handler(stack_frame: &mut InterruptStackFrame){
	(IRQ_HANDLERS.lock()[0])(stack_frame);
	unsafe { PICS.lock().notify_end_of_interrupt(32+0) };
}
pub extern "x86-interrupt" fn irq_1_handler(stack_frame: &mut InterruptStackFrame){
	(IRQ_HANDLERS.lock()[1])(stack_frame);
	unsafe { PICS.lock().notify_end_of_interrupt(32+1) };
}
pub extern "x86-interrupt" fn irq_11_handler(stack_frame: &mut InterruptStackFrame)
{
	(IRQ_HANDLERS.lock()[11])(stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(32+11) };
}

fn default_irq_hand(stack_frame : &mut InterruptStackFrame){
	println!("DEAFAULT IRQ CALLED\n{:#?}",stack_frame);
}

fn timer_interrupt_handler(stack_frame: &mut InterruptStackFrame){
    // print!("."); 
}

fn keyboard_interrupt_handler(stack_frame: &mut InterruptStackFrame)
{
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore) );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }
}

