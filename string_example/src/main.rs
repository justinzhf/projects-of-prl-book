use std::io;

fn main() {
    let mut input = String::new();
    println!("Please input a word.");
    io::stdin().read_line(&mut input).
        expect("Error");
    input = input.trim().to_string();
    println!("{}", convert_str(input));
}

fn convert_str(mut input: String) -> String {
    let first_letter: char = input.remove(0);
    
    match first_letter {
        'a' => first_letter.to_string() + &input + "-hay",
        'e' => first_letter.to_string() + &input + "-hay",
        'i' => first_letter.to_string() + &input + "-hay",
        'o' => first_letter.to_string() + &input + "-hay",
        'u' => first_letter.to_string() + &input + "-hay",
        _  => input + "-" + &first_letter.to_string() + "ay",
    }
}


