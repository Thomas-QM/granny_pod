/*
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
use crate::input_output::*;

// This program is to be run on a Raspberry Pi, providing a voice interface for a Granny Pod. -Brandon

fn main () {
	loop {
		input();
		output();
	}
}