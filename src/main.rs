//Standard libs for i/o
//use std::io;

#[allow(dead_code)]
#[allow(non_snake_case)]
mod input_output;
mod image_rec;
mod voice_rec;
mod help;
use crate::input_output::*;
use crate::help::*;

// declarations; static
static help_detected: bool = false;
static text_sent: bool = false;

fn main () {
	loop {
		input();
		if help_detected { help(); }
		else { }
		output();
	}
}