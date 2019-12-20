#![allow(unused_variables)]
#![allow(dead_code)]

mod front_of_house;

fn server_order() {}

mod back_of_house {
    
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peach"),
            }
        }
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::server_order();
    }
    fn cook_order() {}
}
pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::add_to_waitlist();
 
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Cheese");
    println!("I'd like to eat {:?}.", meal);
    let order1 = back_of_house::Appetizer::Soup;
}

