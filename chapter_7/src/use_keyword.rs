// 7.4 Bringing Paths into Scope with the `use` Keyword
// `use` lets us create a shortcut to a path so we don't have to type
// crate::front_of_house::hosting::serve_order() for example every time we want 
// to call serve_order()
mod front_of_house {
    pub mod hosting {
        pub fn _add_to_waitlist() {}
    }
}

use front_of_house::hosting;

pub fn _eat_at_restaurant() {
    hosting::_add_to_waitlist();
}

// this `use` expression only applies to the scope that it is in
// if, for example, we move the eat_at_restaurant function into a new child
// module, our code will fail to compile because the use and hosting::add_...
// call are not in the same scope anymore

// notice we specified the use path to the parent module, and not to the function itself
// this is the idiomatic way of creating use paths. This makes it clear that the
// hosting::add_to_waitlist() call is not for a locally defined function.
// however, when we are using the `use` keyword to bring in structs, enums, etc
// it is idiomatic to specify the whole path
use std::collections::HashMap;

pub fn run() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("the map: {:?}", map);
}

// no strong reason for this pattern, other than it is the convention that has 
// emerged over time. this pattern differs if we bring 2 items with the same name
// into scope using `use` because Rust won't allow that
/*
use std::fmt;
use std::io;

fn _function1() -> fmt::Result {
    // snip
}

fn _function2() -> io::Result<()> {
    // snip
}
 */
// Both functions return a `Result` from 2 different modules, so they need the parent module
// name included in the path to distinguish them

// We can get around this a bit as well using the `as` keyword to provide a new name or alias
/*
use std::fmt::Result;
use std::io::Result as IoResult;

fn _function1() -> fmt::Result {
    // snip
}

fn _function2() -> IoResult<()> {
    // snip
}
*/
// either of these approaches is fine, so it's up to you what to use

// Re-exporting Names with `pub use`
// When we bring a name into scope with the use keyword, the name available in
//the new scope is private. To enable the code that calls our code to refer to
// that name as if it had been defined in that code’s scope, we can combine pub
// and use.

/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
*/

//Before this change, external code would have to call the add_to_waitlist
// function by using the path restaurant::front_of_house::hosting::add_to_waitlist().
// Now that this pub use has re-exported the hosting module from the root module,
// external code can now use the path restaurant::hosting::add_to_waitlist() instead.

// Using Nested Paths to Clean up Large `use` Lists
/*
instead of:
use std::io;
use std::cmp::Ordering;

we can do:
use std::{cmp::Ordering, io};

Another example
use std::io;
use std::io::Write;
-----
use std::io::{self, Write} <- brings std::io (self) and std::io::Write into scope
*/

// The Glob Operator
// if you want to bring ALL public items from a path into scope we use the glob
// use std::collections::*;
// This is often used in testing to bring everything in to test
// otherwise it can make it difficult to tell where things were imported further down in the code
