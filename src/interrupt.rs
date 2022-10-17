use core::mem::forget;
use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

static TABLE: Mutex<InterruptDescriptorTable> =
	Mutex::new(InterruptDescriptorTable::new());

pub fn init_interrupts() {
	let mut table = TABLE.lock();

	table.invalid_opcode.set_handler_fn(invalid_operation_handler);
	table.double_fault.set_handler_fn(double_fault_hanlder);

	// SAFETY: The 'static in InterruptDescriptorTable::load() only really applies
	// to the data; not the reference. We keep the data valid for 'static here.
	unsafe {table.load_unsafe()}

	// Pass the lock to the CPU.
	forget(table)
}

extern "x86-interrupt" fn invalid_operation_handler(
		stack_frame: InterruptStackFrame) {
	panic!("invalid operation {:?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_hanlder(
		stack_frame: InterruptStackFrame, error_code: u64) -> ! {
	panic!("something bad happened {:?} {:?}", stack_frame, error_code);
}
