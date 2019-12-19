#[derive(Debug)]

struct Rectangle {
	height: u32,
	width: u32,
}
impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
	fn square(size: u32) -> Rectangle {
		Rectangle {height: size, width: size}
	}
}

fn main() {
	let rec = Rectangle {height:3, width:4};
	let sq = Rectangle::square(2);
	
    println!("The rectangel is {:?}", rec);
	println!("Its area is {}", rec.area());
	println!("Can hold? {}", rec.can_hold(&sq));
}
