pub mod gdt;
pub mod pic_init;

pub use x86_64::structures::idt::InterruptDescriptorTable;
pub use x86_64::structures::idt::InterruptStackFrame;
use crate::println;
use crate::print;
use lazy_static::lazy_static;
use pic_init::InterruptIndex;
use pic_init::PICS;

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // CPU Exceptions
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);}

        // Hardware Interrupts
        idt[pic_init::InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[pic_init::InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt[pic_init::InterruptIndex::Rtl8139Int.as_usize()].set_handler_fn(rtl8139_irq_handler);

        idt
    };
}
pub fn init_idt() {
    IDT.load();
}

pub fn init_interrupts(){
    gdt::init();
    init_idt();
    unsafe { pic_init::PICS.lock().initialize()};
    x86_64::instructions::interrupts::enable();
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

//Hardware Interrupt Handlers
pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame)
{
    // print!(".");
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8()) };
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame)
{
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
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

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
pub extern "x86-interrupt" fn rtl8139_irq_handler(stack_frame: &mut InterruptStackFrame)
{
    println!("My IRQ CAlled\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Rtl8139Int.as_u8()) };
}
