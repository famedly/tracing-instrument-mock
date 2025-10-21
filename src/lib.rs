//! TODO: crate documentation

use snafu::Snafu;

/// Library Template error
#[derive(Debug, Snafu)]
pub enum ExampleLibraryError {
	/// Could not find the world to say hello to
	WorldNotFound,
}

/// Write a hello world message
#[allow(clippy::print_stdout)]
pub fn hello_world() -> Result<(), ExampleLibraryError> {
	println!("Hello, world!");
	Ok(())
}
