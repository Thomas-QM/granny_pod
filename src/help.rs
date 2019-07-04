

pub fn help () {
	// basically, if certain stuff happens, set the boolean "help_detected" to true,
	// which will trigger the if statement in the main function

	text(); // this function would also be called to send the text

	// something like:
	// state.help_detected = false;
}

pub fn text () {
	static COUNT: i32 = 0; // amount of times that a text has been sent... might be useful
	// basically code in here will send the text using a text
	// message service, and then set the boolean "text_sent" to true
	// once the text is sent. this will confirm that help has been
	// called, and that help is no longer needed until "help_detected"
	// is true
}