use std::io;

fn main() {
	println!("Please input temperatures. E.g. 32F or 24C");
	let mut input = String::new();
	io::stdin().read_line(&mut input)
		.expect("Illegal format.");
	let trimmed_input = input.trim();
	let (label, input_temp)	= (&trimmed_input[trimmed_input.len()-1..], &trimmed_input[..trimmed_input.len()-1]);
	let input_temp: f32 = input_temp.parse()
		.expect("Illegal format.");

    match label{
		"F" => println!("Your input temperature is {}F. The converted temperature is {}C.", input_temp, (input_temp - 32.0) / 1.8),
		"C" =>  println!("Your input temperature is {}C. The converted temperature is {}F.", input_temp, input_temp * 1.8 + 32.0),
		_ => println!("Illegal format.:wq
		"),
	};
	
}
