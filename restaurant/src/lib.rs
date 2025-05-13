mod front_of_house;

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

pub use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_wishlist();
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_wishlist();
    // crate::front_of_house::hosting::add_to_wishlist(); // Absolute
    // front_of_house::hosting::add_to_wishlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {}

fn function2() -> IoResult<()> {}

use rand::Rng;

fn random_number() {
    rand::thread_rng().gen_range(1..=100)
}

use std::collections::*;
use std::io::{self, Write};
use std::{cmp::Ordering, io};
