//Standard libs for i/o
//use std::io;

/**********************/
//
//	Written by:
//	Brandon Cline
//	
/**********************/

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
mod input_output;
mod image_rec;
mod voice_rec;
mod help;

// global declarations
pub struct State {
	help: help::Help
}

fn main () {
	// struct instantiations
	let state = State {
		help: help::Help::new()
	};

	// maybe some declarations here but doubtful

	loop {
		input();
		if state.help.detected {
			state.help.help();
		}
		
		input_output::output();
	}
}
