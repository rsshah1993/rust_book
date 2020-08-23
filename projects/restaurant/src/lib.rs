#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// Modules:
// modules allow for organization, readibility, and reuse within a crate. 
// Modules are also what control public vs. private parts of your code.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order(){}
        fn serve(){}
        fn take_payment(){}
    }
}

// Paths: 
// paths allow us to find modules within our crate
// path can be absolute and referenced from the crate root (library.rs here)
// relative path start from the current module and uses self or super or identifier in the current module. 

pub fn  eat_at_restaurant(){
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    // we can start with front_of_house since that is at the same level
    // as the current module/function
    front_of_house::hosting::add_to_waitlist();
}


fn serve_order() {}
mod back_of_house {

    // relative paths and super()
    fn cook_order() {}
    fn fix_incorrect_order() {
        cook_order();
        // relative
        super::serve_order();
        // absolute
        crate::serve_order();
    }
    
    // Public structs and enums
    pub struct Breakfast {
        // public struct with public toast field but 
        // private seasonal_fruit field. Fields in a struct
        // are also private by default even if the struct is public.
        pub toast: String,
        seasonal_fruit: String,
    }
    // since seasonal fruit is private we must have a public 
    // association function to construct a Breakfast in a public
    // function (`eat_at_restaurant2`)
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // in contrast to structs, if you make a enum public, all 
    // variants are also public 
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// use with absolute path
// use crate::front_of_house::hosting;

// use with relative path 
use self::front_of_house::hosting;

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // demo all variants of enum are public
    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;

    // use keyword
    // we brought hosting into scope with the `use` key word above. 
    // Now we don't have to specify the absolute or relative path every time.
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// aliasing with as 
use std::fmt::Result;
use std::io::Result as IoResult;

// import from seperate file
mod services;
use crate::services::delivery;

pub fn get_delivery() {
    delivery::wrong_shipment();
}