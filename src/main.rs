#[allow(dead_code)]
#[allow(non_upper_case_globals)]

mod input_output;
mod image_rec;
mod voice_rec;
mod help;

use input_output::*;
use help::*;

// global declarations
pub struct State {
	mut help_detected: bool,
	mut text_sent: bool,
}

fn main () {
	// struct instantiations
	let state = State {
		help_detected: false,
		text_sent: false
	};

	// maybe some declarations here but doubtful

	loop {
		input();
		if state.help_detected {
			help();
		} else {

		}
		output();
	}
}
