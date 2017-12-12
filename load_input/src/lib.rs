
/// Opens the file "input.txt" from the current directory, and 
pub fn load_input() -> String {
	use std::io::Read;
	let args = std::env::args_os();
	let filename = args.skip(1).next().unwrap_or_else(||"input.txt".into());
	let mut result = String::new();
	let mut file = std::fs::File::open(filename).expect("input");
	file.read_to_string(&mut result).expect("input");
	result
}
