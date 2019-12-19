use std::io;

fn main() {
    println!("Please input an integer: ");
	let mut index = String::new();
	io::stdin().read_line(&mut index)
		.expect("Illegal format.");
	let index: u32 = index.trim().parse()
		.expect("Illegal format.");
	println!("The {}th fibonacci number is {}.",index,gen_fibonacci(index));
}

fn gen_fibonacci(index: u32) -> u32{
	let mut a1 = 0;
	let mut a2 = 1;
	for _i in 1..index-1{
		let temp = a2;
		a2 = a1 + a2;
		a1 = temp;	
	}
	a1 + a2
}
