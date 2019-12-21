use std::collections::HashMap;

fn main() {
    let mut nums = vec![3,2,4,5,5,1,6];
    let mut map = HashMap::new();
    let mut sum = 0;
    
    nums.sort();

    for i in &nums {
        sum += i;
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }

    println!("The mean is {}.", sum / nums.len());

    match nums.len() % 2 {
        0 => println!("The median is {}.", (nums[nums.len() / 2 - 1] + nums[nums.len() / 2]) / 2),
        1 => println!("The median is {}.", nums[nums.len() / 2]),
        _ => println!("Error."),
    }
    
    let mut mode = 0;
    for (k, v) in &map {
        match map.get(&mode) {
            Some(count) => {
                if count < v {
                    mode = *k;
                }
            },
            None => mode = *k,
        }
    }
    println!("The mode is {}.", mode);
}
