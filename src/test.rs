use self::super::{vga_buffer::println, harness_main};

pub fn test_main(tests: &[&dyn Fn()]) -> ! {
	println!("Running {} tests", tests.len());
	tests.iter().for_each(|test| test());

	todo!()
}

#[export_name = "_start"]
pub extern "C" fn start() -> ! {
	harness_main();
}
