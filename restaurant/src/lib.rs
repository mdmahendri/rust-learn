// instead of
// use std::io;
// use std::io::Write;
use std::io::{self, Write};

mod front_of_house;

mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        // absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // relative path
        super::front_of_house::hosting::add_to_waitlist();
        // after use keyword
        hosting::add_to_waitlist();

        let order1 = super::back_of_house::Appetizer::Soup;

        let mut meal = super::back_of_house::Breakfast::summer("Wheat");
        meal.toast = String::from("Rye");
        println!("I'd like {} toast please", meal.toast);
    }
}

fn deliver_order() {}

mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
