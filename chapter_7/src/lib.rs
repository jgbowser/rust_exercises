// 7.2 Modules
// lib.rs is a special Rust file. it is not compiled down to an executable like
// a binary crate is.

// lets create a lib module that provides the functionality of a restaurant
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}