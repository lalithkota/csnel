use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use crate::gdt;
use lazy_static::lazy_static;

use crate::pic_init;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // CPU Exceptions
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);}

        // Hardware Interrupts
        idt[pic_init::InterruptIndex::Timer.as_usize()].set_handler_fn(pic_init::timer_interrupt_handler);
        idt[pic_init::InterruptIndex::Keyboard.as_usize()].set_handler_fn(pic_init::keyboard_interrupt_handler);

        idt
    };
}
pub fn init_idt() {
    IDT.load();
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
