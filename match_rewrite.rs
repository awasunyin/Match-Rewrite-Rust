//Simpler implementation of matching 

#[derive(Clone, Copy)]

//Defining the four states of the machine
enum MachineState {

	Normal,
	Comment,
	Upper,
	Lower
}

//Machine State is now Copy
fn machine_cycle(state: MachineState, character: char) -> (Option<char>, MachineState) {
	
	//Giving us lower and uppercase functions
	use std::ascii::AsciiExt;
	use MachineState::*; 

	//Tuple matching
	match (state, character) {

		//When  in normal mode and encountering 'X', 
		//return => None or Some, and state
		(Normal, '#') => (None, Comment), 
		(Normal, '^') => (None, Upper),
		(Normal, '_') => (None, Lower), 

		(Comment, '#') => (None, Normal),
		(Upper, '^') => (None, Normal), 
		(Lower, '_') => (None, Normal),

		(Normal, _) => (Some(character), Normal), 
		(Upper, _) => (Some(character.to_ascii_uppercase()), Upper),
		(Lower, _) => (Some(character.to_ascii_lowercase()), Lower),
		(Comment, _) => (None, Comment),
	} 
}

//Takes input and gives us output
fn run_machine(input: &str) -> String {

	let mut state = MachineState::Normal;
	let mut processed_string = String::new();

	for character in input.chars() {
		let (output, new_state) = machine_cycle(state, character);
		
		//If output is Some rather than None
		if let Some(c) = output {
			processed_string.push(c);
		}
		//State Transition
		state = new_state;
	}

	//Because it is the last expression of the function, 
	//This is what will be ending up returned
	processed_string
} 

fn main() {

	let input_string = "This text is _LOWERCaSe_, this is ^capiTaL. #this is a comment# ";
	let output_string = run_machine(input_string);
	println!("Input: \t{}nOuput: \t{}", input_string, output_string);
}
