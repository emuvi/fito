use std::error::Error;

fn main() { 
	let args: Vec<String> = std::env::args().collect();
	let mut index = 0;
	while index < args.len() {
		let arg = &args[index];

		index += 1;
	}
}

pub type FitoError = Box<dyn Error>;

