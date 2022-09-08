// 6.2 The Match Control Flow Construct

pub fn run() {
    // a good analogy for thinking about `match` is a coin sorting machine
    #[derive(Debug)]
    enum UsState {
        _Alabama,
        _Alaska,
        Utah,
        _Washington,
    }
    enum Coin {
        _Penny,
        _Nickel,
        _Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::_Penny => 1, // each of these are "arms"
            Coin::_Nickel => 5, // pattern to match => code to run
            Coin::_Dime => 10,
            // the state variable added here binds to the value of the Quarter variant (the state of the coin)
            Coin::Quarter(state) => { // if multiple lines, curly braces required
                println!("A Quarter! from {:?}", state);
                25
            },
        }
    }

    let quarter = Coin::Quarter;
    let value = value_in_cents(quarter(UsState::Utah));
    println!("Value: {value}");

    // Patterns that bind to values
    //Another useful feature of match arms is that they can bind to the parts 
    // of the values that match the pattern. This is how we can extract values out of enum variants.
    
    // adding some data to our Quarter variant of Coin above ^^

    // Matching with Option<T>
    // Write a function that takes an Option<i32>, if there is a value, add 1
    // if there isn't a value return `None` and don't perform any operations

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("the value of five is: {:?}, the value of none is: {:?}", five, none);

    // Matches are Exhaustive
    // the arms MUST cover all possibilities, consider the example above
    /*
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x: {
            Some(i) => Some(i + 1),
        }
    }

    this doesn't compile because it doesn't have an arm the addresses the possibility
    of the value being `None`
    */

    // Catch-All Patterns and the _ Placeholder
    // Using enums, we can also take special actions for a few particular values,
    // but for all other values take one default action.

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // catch all, every other value
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(_num_spaces: u8) {}

    /*
    Rust also has a pattern we can use when we want a catch-all but don’t want to
    use the value in the catch-all pattern: _ is a special pattern that matches
    any value and does not bind to that value. This tells Rust we aren’t going to
    use the value, so Rust won’t warn us about an unused variable.
    */

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn reroll() {}

    // nothing happens if you roll anything but 3 or 7
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    } 
}