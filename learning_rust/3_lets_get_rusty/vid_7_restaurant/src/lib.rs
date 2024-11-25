// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

use rand::{CryptoRng, ErrorKind::Transient, Rng};


use std::io;
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// pub fn eat_at_resturant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

fn serve_order() {}

mod back_of_house_two {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order()
    }

    fn cook_order() {}
}

// Privacy rules with structs

mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,      // Public field
        seasonal_fruit: String, // Private field
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_resturant() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut meal = back_of_house::Breakfast::summer("toast");
    meal.toast = String::from("Wheat");
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // The use of the `use` keyward
    hosting::add_to_waitlist();
}
