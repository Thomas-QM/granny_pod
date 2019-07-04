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
use crate::input_output::*;
use crate::help::*;

// global declarations
pub struct State {
	mut help_detected: bool,
	mut text_sent: bool,
}

fn main () {
	// structs
	let state = State {
		help_detected: false,
		text_sent: false
	};

	loop {
		input();
		if state.help_detected {
			help();
		} else {

		}
		output();
	}
}