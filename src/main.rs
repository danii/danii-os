//! An adventure in making my own operating system.

#![feature(
	abi_x86_interrupt,
	const_trait_impl,
	custom_test_frameworks,
	decl_macro
)]
#![test_runner(self::test::test_main)]
#![reexport_test_harness_main = "harness_main"]
#![no_std]
#![no_main]

#![warn(
	missing_docs,
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	clippy::undocumented_unsafe_blocks
)]
#![allow(
	// Rationale: I'd imagine that if this was a problem it would create type
	// issues.
	clippy::unit_arg
)]

#[cfg(not(test))]
mod boot;
mod interrupt;
//#[cfg(test)]
//mod test;
mod vga_buffer;
