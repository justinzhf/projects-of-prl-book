use std::collections::HashMap;
use std::io::{self,Write};

fn main() {
    let mut map: HashMap<String, String> = HashMap::new();
    println!("Add an employee: add Tom to Engineering");
    println!("List all employees of a department: list Engineering");
    println!("List all employees: list all");
    loop {
        let mut input = String::new();
        print!(">");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input)
            .expect("Error");
        input = input.trim().to_string();
        if input == "list all" {
            println!("{:?}", list_all_employees(&map));
        }else if &input[0..3] == "add" {
            add_employee(input, &mut map);
        }else {
            println!("{:?}", list_employees_by_department(input, &map));
        }
    }
}

fn add_employee(input: String, map: &mut HashMap<String, String>) {
    let words: Vec<&str> = input.split(' ').collect();
    map.insert(words[1].to_string(), words[3].to_string());
}

fn list_employees_by_department(input: String, map: &HashMap<String, String>) -> Vec<&str> {
    let words: Vec<&str> = input.split(' ').collect();
    let mut employees: Vec<&str> =Vec::new();
    for (k, v) in map {
        if v == words[1] {
            employees.push(k);
        }
    }

    employees.sort();
    employees
}

fn list_all_employees(map: &HashMap<String, String>) -> Vec<(String, String)> {
    let mut employees: Vec<(String, String)> = Vec::new();
    let tmp = String::from("");
    for (k, v) in map {
        employees.push((v.to_string(), k.to_string()));
    }
    employees.sort_by_key(|k| tmp.clone() + &k.0 + &k.1);
    employees
}
