#![allow(
	// Rationale: This is more so like an internal library.
	unused
)]

use core::{fmt::{Write, Result as FMTResult, Display}, marker::PhantomData};
use spin::Mutex;
use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const VGA_BUFFER: *mut Buffer = 0xB8000 as *mut Buffer;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ColorSet(u8);

impl ColorSet {
	pub const fn new(foreground: Color, background: Color) -> Self {
		Self((background as u8) << 4 | foreground as u8)
	}
}

impl const Default for ColorSet {
	fn default() -> Self {
		Self::new(Color::LightGray, Color::Black)
	}
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Char {
	character: u8,
	appearance: ColorSet
}

impl Char {
	const fn new(character: u8, appearance: ColorSet) -> Self {
		Self {character, appearance}
	}
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct Buffer([Char; BUFFER_WIDTH * BUFFER_HEIGHT]);

pub struct Output {
	position: usize,
	style: ColorSet,
	buffer: PhantomData<*mut Buffer>
}

impl Output {
	/// Safety
	/// ------
	/// Output is a singleton, and only one instance can exist safely.
	const unsafe fn new() -> Self {
		Self {position: 0, style: ColorSet::default(), buffer: PhantomData}
	}

	/// The [Mutex] that Output typically sits in protects this [Buffer]
	/// reference; hence why the reference lasts as long as the reference to self,
	/// and not `'static`.
	fn buffer(&mut self) -> Volatile<&mut Buffer> {
		// SAFETY: 0xB8000 is memory mapped I/O for the VGA text buffer.
		unsafe {Volatile::new(&mut *VGA_BUFFER)}
	}

	pub fn r#in(&mut self, style: ColorSet) {
		self.style = style;
	}

	pub fn write_byte(&mut self, byte: u8) {
		// Create a new line as needed.
		if self.position >= BUFFER_WIDTH * BUFFER_HEIGHT {self.scroll(1);}

		// Commit the character.
		let Self {position, style, ..} = *self;
		let character = Char::new(byte, style);
		self.buffer().update(|buffer| buffer.0[position] = character);

		// Forward the position.
		self.position += 1;
	}

	pub fn write_str(&mut self, string: &str) {
		string.bytes()
			.for_each(|byte| match byte {
				// These are the only UTF-8 characters that are also available in code
				// page 437.
				byte @ 0x20..=0x7E => self.write_byte(byte),

				// Newline does have a representation in the VGA text buffer, but it's
				// not a character.
				b'\n' => self.new_line(),

				// We use the black square character to represent non printable
				// characters.
				_ => self.write_byte(0xFE)
			});
	}

	pub fn new_line(&mut self) {
		self.position = (self.position / BUFFER_WIDTH + 1) * BUFFER_WIDTH;
	}

	pub fn scroll(&mut self, lines: usize) {
		for row in lines..BUFFER_HEIGHT {
			for column in 0..BUFFER_WIDTH {
				let current = row * BUFFER_WIDTH + column;
				let previous = (row - lines) * BUFFER_WIDTH + column;

				let character = self.buffer().read().0[current];
				self.buffer().update(|buffer| buffer.0[previous] = character);
			}
		}

		self.position -= lines * BUFFER_WIDTH;
	}

	pub fn set_cursor(&mut self, row: usize, column: usize) {
		self.position = row * BUFFER_WIDTH + column;
	}
}

impl Write for Output {
	fn write_str(&mut self, string: &str) -> FMTResult {
		Ok(self.write_str(string))
	}
}

// SAFETY: We use the buffer pointer responsibly.
unsafe impl Send for Output {}

// SAFETY: This is the only single instance of Writer.
#[allow(
	// RATIONALE: It's there. I don't know why clippy is complaining.
	clippy::undocumented_unsafe_blocks
)]
pub static OUTPUT: Mutex<Output> = unsafe {Mutex::new(Output::new())};

pub macro print($($arg:tt)*) {
	OUTPUT.lock()
		.write_fmt(format_args!($($arg)*))
		.expect("is always OK")
}

pub macro println {
	() => {print!("\n")},
	($($arg:tt)*) => {print!("{}\n", format_args!($($arg)*))}
}
