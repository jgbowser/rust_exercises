// 7.2 Modules
// lib.rs is a special Rust file. it is not compiled down to an executable like
// a binary crate is.

// lets create a lib module that provides the functionality of a restaurant
mod front_of_house {
    pub mod hosting {
        pub fn _add_to_waitlist() {}
        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}
        fn _serve_order() {}
        fn _take_payment() {}
    }
}

// // Two ways to call the add to waitlist function: relative and absolute
// pub fn eat_at_restaurant() {
//     // Absolute Path
//     crate::front_of_house::hosting::add_to_waitlist(); // start with `crate` because we are in the file/crate

//     // relative Path
//     front_of_house::hosting::add_to_waitlist();
// }

// // Starting a relative path with the `super` keyword is kind of like starting a
// // path with `~`. It allows us to start from the crate root.
// fn deliver_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order(); // super brings us up to the parent level, basically the same as `crate::` in this case
//     }
//     fn cook_order() {}
// }

// Making structs and enums public
// commenting out the code above since this example uses the same names
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        // because there is a private field we need a pub fn to to construct an
        // instance of Breakfast, otherwise we wouldn't be able to set the private
        // seasonal_fruit field
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye Toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // With structs we have to individually make fields public, even if the
    // struct itself is public
    // this is different with enums, if you mark an enum as public
    //all it's variants will also be public
    // see the Appetizer enum in back_of_house above
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}