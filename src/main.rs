/*
Standard libs for i/o and random number generation
use std::io;
use std::cmp::Ordering;
use rand::Rng;
*/

#[allow(dead_code)]
#[allow(non_snake_case)]
mod declarations;
mod input_output;
mod image_rec;
mod voice_rec;
mod help;
use crate::input_output::*;
use crate::help::*;

// This program is to be run on a Raspberry Pi, providing a voice interface for a Granny Pod. -Brandon

fn main () {
	loop {
		input();
		if help_detected { help(); }
		else
		output();
	}
}