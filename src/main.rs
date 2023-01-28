use std::env;
use std::fs::read_to_string;

mod router;
mod value;
mod s_exp_parser;

fn main() {
	let mut to_stdout = false;

	let argv : Vec<String> = env::args().collect();
	if argv.len() < 2 {()} // return if no input and output file is given
	if argv.len() < 3 {
		to_stdout = true;
	}

	let input_file = read_to_string(&argv[1]).unwrap();

	let input_board = s_exp_parser::parse(&input_file).unwrap();

	println!("{:?}", input_board);

	//let output_board = router::route(input_board);

	if to_stdout {
//		println!("{:?}", output_board);
	} else {
		// write to file
	}
}