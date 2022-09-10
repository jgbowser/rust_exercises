// 6.3 Concise Control Flow with `if let`

pub fn _run() {
    // if let is a shorthand way to handle values that match one pattern and ignore the rest
    // example:
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (), // <-- this is kind of useless boiler plate
    }
    // In this example we only do something with the Some case, otherwise we do
    // nothing (_ => ()), this extra boilerplate is annoying, and something we
    // can get rid of thanks to if let
    if let Some(max) = config_max {
        // if let <pattern> = <expression>
        println!("The maximum is configured to be {} (if let)", max);
    }
    // this is more concise, but it also means you lose some of the exhaustive
    // checking that `match` enforces, so using it is a bit of a balancing act.

    // We can also add an `else` block to an `if let`. let's look at an example
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
        Dime,
        Quarter(UsState),
    }
    let coin1 = Coin::Quarter(UsState::Utah);
    let coin2 = Coin::Dime;
    fn coin_check(coin: Coin) {
        let mut count = 0;
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}", state),
            _ => count += 1,
        }
        println!("count: {count}");
    }

    coin_check(coin1);
    coin_check(coin2);

    // this could be an if let with an else block though
    /*
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
    */
}
