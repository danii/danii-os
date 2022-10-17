use crate::interrupt;

use self::super::vga_buffer::{OUTPUT, ColorSet, Color, println};
use core::{fmt::Write, panic::PanicInfo, sync::atomic::{AtomicUsize, Ordering}};
use x86_64::instructions::hlt;

fn exit() -> ! {
	loop {hlt()};
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	static PANIC_COUNT: AtomicUsize = AtomicUsize::new(0);

	match PANIC_COUNT.fetch_add(1, Ordering::Relaxed) {
		// No other panics occured; perform the normal panic routine.
		0 => {
			// SAFETY: Not much reasoning here. Would you prefer the chance at knowing
			// what went wrong or no chance at all?
			unsafe {OUTPUT.force_unlock()};
			let mut writer = OUTPUT.lock();
			writer.set_cursor(0, 0);
			writer.r#in(ColorSet::new(Color::White, Color::Red));
			let _ = writeln!(writer, "{}", info);
		},

		// Double panic; try to at least communicate this.
		1 => {
			// SAFETY: See above.
			unsafe {OUTPUT.force_unlock()}
			let mut writer = OUTPUT.lock();
			writer.write_str("double panic");
		},

		// Triple panic; give up.
		_ => ()
	}

	exit()
}

#[export_name = "_start"]
pub extern "C" fn start() -> ! {
	interrupt::init_interrupts();
	println!("Hello World!");
	exit()
}