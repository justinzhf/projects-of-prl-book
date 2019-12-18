use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("secret number {}",secret_number);
    loop {
	    println!("Please input the number you guessed:");

	    let mut guess = String::new();
	    io::stdin().read_line(&mut guess)
		    .expect("The input number is illegal!");
	    let guess: u32 = guess.trim().parse()
		    .expect("Please type a number!");

	    println!("The number you guess is {}",guess);

	    match guess.cmp(&secret_number){
		    Ordering::Less => println!("The number you guessed is too small!"),
			    Ordering::Greater => println!("The number you guessed is too large!"),
			    Ordering::Equal =>{
					println!("You win!");
					break;
				}
	    }
    }

}
