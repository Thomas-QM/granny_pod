#[derive(Debug)]
struct Help {
	detected: bool,
	count: usize //use unsigned integers for >0 values
}

impl Help {
	pub fn new() -> Self {
		Help { detected: false, count: 0 }
	}
	
	pub fn help (&mut self) {
		// basically, if certain stuff happens, set the boolean "help_detected" to true,
		// which will trigger the if statement in the main function

		self.text(); // this function would also be called to send the text

		// something like:
		self.help_detected = false;
	}

	pub fn text (&mut self) {
		self.text += 1;

		// basically code in here will send the text using a text
		// message service, and then set the boolean "text_sent" to true
		// pls stop using so many bools
		// once the text is sent. this will confirm that help has been
		// called, and that help is no longer needed until "help_detected"
		// is true

		// this will need to incorporate the image hosting service
		// as well as the text hosting service
	}
}
