use std::error::Error;

mod clip;
mod mirror;

pub type FitoError = Box<dyn Error>;

fn main() -> Result<(), FitoError> {
    let args = clip::parse();
    let speed_str = args.value_of("speed").expect("You must pass a speed.");
	let speed: usize = speed_str.parse().expect("You must pass a valid speed.");
    let input = args.value_of("input").expect("You must pass the input path.");
    let output = args.value_of("output").expect("You must pass the output path.");
    if args.is_present("mirror") {
        mirror::make(speed, input, output)?;
    }
    Ok(())
}
